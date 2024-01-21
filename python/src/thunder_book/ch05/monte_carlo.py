import numpy as np
import fire

from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.random_action import random_action
from thunder_book.ch05.average_score import average_score


def playout(state: State) -> float:
    # base case
    if state.is_done():
        return state.teban_score()

    # recursion without copy
    state.advance(random_action(state))
    return 1 - playout(state)


def primitive_monte_carlo_action(state: State, playout_number: int) -> int:
    legal_actions = state.legal_actions()

    N = len(legal_actions)
    values = np.zeros(N)
    counts = np.zeros(N)

    for i in range(playout_number):
        index = i % N
        next_state = state.copy()
        next_state.advance(legal_actions[index])
        values[index] += 1 - playout(next_state)
        counts[index] += 1

    # get best action (argmax of values / counts)
    average_scores = values / counts
    best_action_index = np.argmax(average_scores)
    return legal_actions[best_action_index]


def play_monte_carlo_vs_random(a: int = 30):
    print(f"monte carlo {a} vs. random")
    monte_carlo_action_f = lambda state: primitive_monte_carlo_action(state, a)
    win_rate = average_score(100, (monte_carlo_action_f, random_action))
    print(f"win rate of monte carlo {a} vs. random: {win_rate:.2f}")


def compare_monte_carlo(a: int = 10, b: int = 3):
    print(f"compare monte carlo {a} vs. {b}")
    monte_carlo_action_a = lambda state: primitive_monte_carlo_action(state, a)
    monte_carlo_action_b = lambda state: primitive_monte_carlo_action(state, b)
    win_rate = average_score(100, (monte_carlo_action_a, monte_carlo_action_b))
    print(f"win rate of monte carlo {a} vs. {b}: {win_rate:.2f}")


def main(game="compare", *args, **kwargs):
    if game == "compare":
        compare_monte_carlo(*args, **kwargs)
    if game == "play":
        play_monte_carlo_vs_random(*args, **kwargs)


if __name__ == "__main__":
    fire.Fire(main)
