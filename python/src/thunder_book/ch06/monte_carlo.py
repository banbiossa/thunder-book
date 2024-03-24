import logging
from datetime import datetime

import fire
import numpy as np

from thunder_book.ch06.game import many_games
from thunder_book.ch06.maze_state import ActionFunc
from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.random_action import random_action
from thunder_book.util import setup_logging


class Playout:
    def __init__(self, state: State) -> None:
        self.state = state.copy()

    def playout(self, player_id: int) -> float:
        if self.state.is_done():
            return self.state.score(player_id)

        self.state.advance(
            random_action(self.state, 0),
            random_action(self.state, 1),
        )
        return self.playout(player_id)


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

            value += Playout(next_state).playout(player_id)

        values.append(value)

    # get argmax of values
    best_index = np.argmax(values)
    return legal_actios[best_index]


def make_monte_carlo_f(playout_number: int) -> ActionFunc:
    def monte_carlo_f(state: State, player_id: int) -> int:
        return monte_carlo_action(state, player_id, playout_number)

    return monte_carlo_f


def monte_carlo_vs_random(playout_number=10, num_games=100):
    file_logger = logging.getLogger("file_logger")
    monte_carlo_f = make_monte_carlo_f(playout_number)
    actions_wb = (monte_carlo_f, random_action)

    start = datetime.now()
    win_rate = many_games(num_games, actions_wb, player_id=0, print_every=10)
    elapsed = (datetime.now() - start).total_seconds()

    print()
    print(f"{win_rate=:.2f} for monte carlo {playout_number} vs random")
    file_logger.info(
        f"| monte carlo {playout_number} vs random | {win_rate*100:.2f}% | {elapsed:.2f} |"
    )


def main():
    file_logger = logging.getLogger("file_logger")
    file_logger.info("| name | score | time |")
    file_logger.info("| ---- | ----- | ---- |")
    monte_carlo_vs_random()


if __name__ == "__main__":
    setup_logging()
    fire.Fire(main)
