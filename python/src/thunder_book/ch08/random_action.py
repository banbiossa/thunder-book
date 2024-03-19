import random

from thunder_book.ch08.maze_state import ConnectFourState


def random_action(state: ConnectFourState) -> int:
    return random.choice(state.legal_actions())
