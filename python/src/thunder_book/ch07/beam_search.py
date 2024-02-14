from thunder_book.ch07 import constants as C
from thunder_book.ch07.game import play_game, white_games
from thunder_book.ch07.maze_state import WallMazeState as State


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
        0,
    )


def play_many_beam_search(use_zobrist_hash: bool):
    depth = C.END_TURN
    width = 100
    num_games = 10

    print(f"beam search {depth=}, {width=}, {num_games=}, {use_zobrist_hash=}")
    score = white_games(
        make_beam_search_f(
            depth=depth,
            width=width,
            use_zobrist_hash=use_zobrist_hash,
        ),
        num_games=num_games,
        print_every=1,
    )
    print("average score:", score)


if __name__ == "__main__":
    play_many_beam_search(False)
    play_many_beam_search(True)
