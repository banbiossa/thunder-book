import random
from datetime import datetime

from thunder_book.ch07.game import BeamType, get_state, play_game, white_games
from thunder_book.ch07.maze_state import MazeParams, State


def beam_search_action(
    initial_state: State,
    depth: int,
    width: int,
    use_zobrist_hash: bool,
) -> int:
    beam: list[State] = [initial_state.copy()]
    best_state = initial_state
    hash_check = set()

    for d in range(depth):
        next_beam: list[State] = []
        for _ in range(width):
            if not beam:
                break
            beam.sort()
            state = beam.pop()

            legal_actions = state.legal_actions()
            for action in legal_actions:
                next_state = state.copy()
                next_state.advance(action)
                # hash check
                if use_zobrist_hash and d >= 1 and next_state.hash in hash_check:
                    continue
                hash_check.add(next_state.hash)

                next_state.evaluate_score()
                if d == 0:
                    next_state.first_action = action
                next_beam.append(next_state.copy())
                if not next_state.is_legal():
                    breakpoint()

        beam = sorted(next_beam)
        best_state = beam[-1].copy()
        if best_state.is_done():
            break

    assert best_state.first_action != -1
    assert best_state.first_action in initial_state.legal_actions()
    return best_state.first_action


def make_beam_search_f(
    *,
    depth: int,
    width: int,
    use_zobrist_hash: bool,
):
    def beam_search_f(state: State) -> int:
        return beam_search_action(
            state,
            depth,
            width,
            use_zobrist_hash,
        )

    return beam_search_f


def play_beam_search():
    play_game(
        make_beam_search_f(
            depth=100,
            width=4,
            use_zobrist_hash=False,
        ),
        seed=0,
        params=MazeParams(height=7, width=7, end_turn=49),
        beam_type=BeamType.normal,
    )


def play_many_beam_search(
    *,
    use_zobrist_hash: bool,
    beam_type: BeamType,
    num_games=10,
    width=100,
):
    params = MazeParams(height=7, width=7, end_turn=49)
    depth = params.end_turn

    print(f"beam search {depth=}, {width=}, {num_games=}, {use_zobrist_hash=}, {beam_type=}")
    score = white_games(
        make_beam_search_f(
            depth=depth,
            width=width,
            use_zobrist_hash=use_zobrist_hash,
        ),
        params=params,
        num_games=num_games,
        print_every=1,
        beam_type=beam_type,
    )
    print("average score:", score)


def time_many_beam_search(
    *,
    game_number: int = 10,
    per_game: int = 100,
    print_every: int = 1,
    use_zobrist_hash: bool = False,
    beam_type: BeamType,
):
    print(f"beam search time {game_number=}, {per_game=}, {use_zobrist_hash=}, {beam_type=}")
    params = MazeParams(height=7, width=7, end_turn=49)
    diff_sum = 0
    random.seed(0)
    for i in range(game_number):
        state = get_state(random.randint(0, 2**16 - 1), beam_type, params)
        start_time = datetime.now()
        for j in range(per_game):
            beam_search_action(state, 100, 4, use_zobrist_hash)
        diff = datetime.now() - start_time
        diff_sum += diff.total_seconds()
        if print_every > 0 and i % print_every == 0:
            print(f"{i=}, {diff_sum*1000/(i+1):.2f}ms")
    time_mean = diff_sum / game_number
    print(f"beam search time mean: {time_mean*1000:.2f}ms")


if __name__ == "__main__":
    play_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.numpy)
    time_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.numpy)

    play_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.single)
    time_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.single)

    play_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.multi)
    time_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.multi)

    play_many_beam_search(use_zobrist_hash=False, beam_type=BeamType.normal)
    time_many_beam_search(use_zobrist_hash=False, beam_type=BeamType.normal)

    play_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.normal)
    time_many_beam_search(use_zobrist_hash=True, beam_type=BeamType.normal)
