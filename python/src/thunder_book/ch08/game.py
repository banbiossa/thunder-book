from thunder_book.ch08.maze_state import ActionFunc, ConnectFourState, MazeParams
from thunder_book.ch08.random_action import random_action


def play_game(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    should_print: bool = False,
) -> float:
    state = ConnectFourState(params)

    if should_print:
        print(state.to_string())

    while not state.is_done():
        if should_print:
            print(state.to_string())

    return state.white_score()


if __name__ == "__main__":
    play_game(
        MazeParams(width=7, height=6),
        (random_action, random_action),
        should_print=True,
    )
