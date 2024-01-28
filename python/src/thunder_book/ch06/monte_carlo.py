import numpy as np

from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.random_action import random_action


def playout(state: State, player_id: int):
    if state.is_done():
        return state.score(player_id)

    state.advance(
        random_action(state, 0),
        random_action(state, 1),
    )
    return playout(state, player_id)


def monte_carlo_action(state: State, player_id: int, playout_number: int) -> int:
    legal_actios = state.legal_actions(player_id)
    opponent_id = player_id ^ 1
    values = []

    for action in legal_actios:
        value = 0
        for _ in range(playout_number):
            opponent_action = random_action(state, opponent_id)

            next_state = state.copy()
            if player_id == 0:
                next_state.advance(action, opponent_action)
            else:
                next_state.advance(opponent_action, action)

            value += playout(next_state, player_id)

        values.append(value)

    # get argmax of values
    best_index = np.argmax(values)
    return legal_actios[best_index]
