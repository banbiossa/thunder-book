import random

from thunder_book.ch08.maze_state import MazeState


def random_action(state: MazeState) -> int:
    return random.choice(state.legal_actions())
