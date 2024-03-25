import random

from thunder_book.ch07.game import BeamType, play_game
from thunder_book.ch07.maze_state import MazeParams, State

random.seed(0)


def random_action(state: State) -> int:
    legal_actions = state.legal_actions()
    return random.choice(legal_actions)


def play_random():
    play_game(
        random_action,
        0,
        MazeParams(height=7, width=7, end_turn=49),
        BeamType.normal,
    )


if __name__ == "__main__":
    play_random()
