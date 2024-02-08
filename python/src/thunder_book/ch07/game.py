from thunder_book.ch07.maze_state import ActionFunc
from thunder_book.ch07.maze_state import WallMazeState as State


def play_game(action_func: ActionFunc, seed: int) -> None:
    state = State(seed)
    print(state)
    while not state.is_done():
        action = action_func(state)
        state.advance(action)
        print(state)
