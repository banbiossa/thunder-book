from typing import Callable

from thunder_book.ch05 import constants
from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.mini_max import mini_max_action
from thunder_book.ch05.random_action import random_action


def play_game(seed: int = 0) -> None:
    state = State(seed=seed)
    print(state)
    actions = [
        lambda state: mini_max_action(state, 2),
        random_action,
    ]
    p = 0

    while not state.is_done():
        print(f"{p+1}p {'-'*20}")
        action = actions[p](state)
        print(f"action:\t{action}")
        state.advance(action)
        print(state)
        p ^= 1
    state.print_end_game()


ActionFunc = Callable[[State], int]


def one_game(seed: int, actions: list[ActionFunc]):
    state = State(seed=seed)
    p = 0
    while not state.is_done():
        action = actions[p](state)
        state.advance(action)
        p ^= 1
    return state.white_score()


def average_score(num_games: int, do_print=True) -> float:
    score = 0

    mini_max_action_f = lambda state: mini_max_action(state, constants.END_TURN)
    actions_bw = [mini_max_action_f, random_action]
    actions_wb = [random_action, mini_max_action_f]

    for i in range(num_games):
        score += one_game(i, actions_bw)
        score += 1 - one_game(i, actions_wb)

        if do_print and i % 10 == 0:
            tmp_avg = score / 2 / (i + 1)
            print(f"{i=} {tmp_avg:.2f}")

    return score / 2 / num_games


if __name__ == "__main__":
    score = average_score(100)
    print(f"score: {score:.2f}")
