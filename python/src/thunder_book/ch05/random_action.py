import random

import fire

from thunder_book.ch05.maze_state import AlternateMazeState, MazeParams


def random_action(state: AlternateMazeState) -> int:
    legal_actions = state.legal_actions()
    return random.choice(legal_actions)


def play_game(seed: int = 0) -> None:
    state = AlternateMazeState(seed=seed, params=MazeParams(height=3, width=4, end_turn=10))
    print(state)
    p = 0
    while not state.is_done():
        print(f"{p+1}p {'-'*20}")
        action = random_action(state)
        print(f"action:\t{action}")
        state.advance(action)
        print(state)
        p ^= 1
    state.print_end_game()


if __name__ == "__main__":
    fire.Fire(play_game)
