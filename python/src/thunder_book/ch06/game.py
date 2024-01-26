from typing import Callable

import fire

from thunder_book.ch06 import constants as C
from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.random_action import random_action

# State & player_id -> action
ActionFunc = Callable[[State, int], int]


def play_game(action_f0: ActionFunc, action_f1: ActionFunc, seed: int):
    state = State(seed=seed)
    print(state)

    while not state.is_done():
        action0 = action_f0(state, 0)
        action1 = action_f1(state, 1)
        print(f"actions {C.dtor[action0]} {C.dtor[action1]}")
        state.advance(action0, action1)
        print(state)


def random_vs_random():
    play_game(random_action, random_action, seed=0)


if __name__ == "__main__":
    fire.Fire(random_vs_random)
