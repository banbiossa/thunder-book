import random
from datetime import datetime
from typing import Callable

from thunder_book.ch05 import constants
from thunder_book.ch05.alpha_beta import alpha_beta_action
from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.mini_max import mini_max_action


def get_sample_states(game_number: int) -> list[State]:
    states: list[State] = []
    random.seed(0)
    seeds = [random.randint(0, 10000) for _ in range(game_number)]
    for i in range(game_number):
        state = State(seed=seeds[i])
        turn = random.randint(0, constants.END_TURN)
        for _ in range(turn):
            state.advance(random.choice(state.legal_actions()))
        states.append(state.copy())

    return states


ActionFunc = Callable[[State], int]


def calculate_execution_speed(
    name, action_func: ActionFunc, states: list[State]
) -> None:
    start = datetime.now()
    for state in states:
        action_func(state.copy())
    duration = (datetime.now() - start).total_seconds() * 1000

    print(f"{name}: {duration:.2f}ms for {len(states)} states")


def compare_alpha_beta_to_mini_max() -> None:
    states = get_sample_states(100)
    alpha_beta_action_f = lambda state: alpha_beta_action(state, constants.END_TURN)
    calculate_execution_speed("alpha_beta", alpha_beta_action_f, states)

    states = get_sample_states(100)
    mini_max_action_f = lambda state: mini_max_action(state, constants.END_TURN)
    calculate_execution_speed("mini_max", mini_max_action_f, states)


if __name__ == "__main__":
    compare_alpha_beta_to_mini_max()
