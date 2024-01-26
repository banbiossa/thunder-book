import random

from thunder_book.ch06.maze_state import SimulataneousMazeState as State


def random_action(state: State, player_id: int) -> int:
    legal_actions = state.legal_actions(player_id)
    return random.choice(legal_actions)
