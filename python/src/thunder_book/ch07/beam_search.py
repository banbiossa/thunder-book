from thunder_book.ch07 import constants as C
from thunder_book.ch07.game import play_game, white_games
from thunder_book.ch07.maze_state import WallMazeState as State


def beam_search_action(state: State, depth: int, width: int) -> int:
    now_beam: list[State] = [state.copy()]
    best_state = state.copy()

    for d in range(depth):
        next_beam: list[State] = []
        for _ in range(width):
            if not now_beam:
                break
            now_beam.sort()
            now_state = now_beam.pop()

            legal_actions = now_state.legal_actions()
            for action in legal_actions:
                next_state = now_state.copy()
                next_state.advance(action)
                next_state.evaluate_score()
                if d == 0:
                    next_state.first_action = action
                next_beam.append(next_state.copy())
                if not next_state.is_legal():
                    breakpoint()

        now_beam = sorted(next_beam)
        best_state = now_beam[-1].copy()
        if best_state.is_done():
            break

    assert best_state.first_action != -1
    assert best_state.first_action in state.legal_actions()
    return best_state.first_action


def make_beam_search_f(*, depth: int, width: int):
    def beam_search_f(state: State) -> int:
        return beam_search_action(state, depth, width)

    return beam_search_f


def play_beam_search():
    play_game(make_beam_search_f(depth=100, width=4), 0)


def play_many_beam_search():
    depth = C.END_TURN
    width = 100
    num_games = 100

    print(f"beam search depth: {depth}, width: {width}, num_games: {num_games}")
    score = white_games(
        make_beam_search_f(depth=depth, width=width),
        num_games=num_games,
        print_every=1,
    )
    print("average score:", score)


if __name__ == "__main__":
    play_many_beam_search()
