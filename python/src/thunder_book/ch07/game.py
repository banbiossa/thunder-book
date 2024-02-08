from thunder_book.ch07.maze_state import ActionFunc
from thunder_book.ch07.maze_state import WallMazeState as State

ACTION_TO_STR = ["RIGHT", "LEFT", "DOWN", "UP"]


def play_game(action_func: ActionFunc, seed: int) -> None:
    state = State(seed)
    print(state)
    while not state.is_done():
        action = action_func(state)
        print(f"action: {ACTION_TO_STR[action]}")
        state.advance(action)
        print(state)
