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


def white_games(action_func: ActionFunc, num_games: int, print_every: int = 10) -> float:
    total = 0
    for i in range(num_games):
        state = State(i)
        while not state.is_done():
            action = action_func(state)
            state.advance(action)
        total += state.game_score
        if i % print_every == 0:
            print(f"game {i} score: {total/(i+1):.1f}")

    return total / num_games
