from __future__ import annotations

import fire
import numpy as np

from thunder_book.ch05.average_score import average_score
from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.maze_state import MazeParams
from thunder_book.ch05.random_action import random_action

# def playout(state: State) -> float:
#     # base case
#     if state.is_done():
#         return state.teban_score()

#     # recursion without copy
#     state.advance(random_action(state))
#     return 1 - playout(state)


class Playout:
    def __init__(self, state: State) -> None:
        """上の playout でstate を渡す際にcopyしなかったバグを
        踏んだので, 絶対に copy するための工夫。

        use like Playout(state).advance(action).playout()
        """
        self.state = state.copy()

    def advance(self, action: int) -> Playout:
        self.state.advance(action)
        return self

    def playout(self) -> float:
        if self.state.is_done():
            return self.state.white_score()

        self.state.advance(random_action(self.state))
        return 1 - self.playout()


def primitive_monte_carlo_action(state: State, num_playout: int) -> int:
    legal_actions = state.legal_actions()

    N = len(legal_actions)
    values = np.zeros(N)
    counts = np.zeros(N)

    for i in range(num_playout):
        index = i % N
        action = legal_actions[index]
        values[index] += 1 - Playout(state).advance(action).playout()
        counts[index] += 1

    # get best action (argmax of values / counts)
    average_scores = values / counts
    best_action_index = np.argmax(average_scores)
    return legal_actions[best_action_index]


def play_monte_carlo_vs_random(num_playout: int = 300):
    print(f"monte carlo {num_playout} vs. random")
    params = MazeParams(width=5, height=5, end_turn=10)
    monte_carlo_action_f = lambda state: primitive_monte_carlo_action(state, num_playout)
    win_rate = average_score(100, (monte_carlo_action_f, random_action), params=params)
    print(f"win rate of monte carlo {num_playout} vs. random: {win_rate:.2f}")


def compare_monte_carlo(a: int = 10, b: int = 3):
    print(f"compare monte carlo {a} vs. {b}")
    params = MazeParams(width=5, height=5, end_turn=10)
    monte_carlo_action_a = lambda state: primitive_monte_carlo_action(state, a)
    monte_carlo_action_b = lambda state: primitive_monte_carlo_action(state, b)
    win_rate = average_score(100, (monte_carlo_action_a, monte_carlo_action_b), params=params)
    print(f"win rate of monte carlo {a} vs. {b}: {win_rate:.2f}")


def main(game="all", *args, **kwargs):
    if game == "compare":
        compare_monte_carlo(*args, **kwargs)
    if game == "play":
        play_monte_carlo_vs_random(*args, **kwargs)
    if game == "all":
        compare_monte_carlo()
        play_monte_carlo_vs_random()


if __name__ == "__main__":
    fire.Fire(main)
