from typing import Callable

import fire

from thunder_book.ch05.alpha_beta import alpha_beta_action
from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.maze_state import MazeParams
from thunder_book.ch05.mini_max import mini_max_action
from thunder_book.ch05.random_action import random_action


def play_game(seed: int, params: MazeParams) -> None:
    state = State(seed=seed, params=params)
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


type ActionFunc = Callable[[State], int]


def one_game(seed: int, actions: tuple[ActionFunc, ActionFunc], params: MazeParams):
    state = State(seed=seed, params=params)
    p = 0
    while not state.is_done():
        action = actions[p](state)
        state.advance(action)
        p ^= 1
    return state.white_score()


def average_score(
    num_games: int,
    actions_wb: tuple[ActionFunc, ActionFunc],
    params: MazeParams,
    do_print=True,
) -> float:
    score = 0
    actions_bw = (actions_wb[1], actions_wb[0])

    for i in range(num_games):
        score += one_game(i, actions_wb, params=params)
        score += 1 - one_game(i, actions_bw, params=params)

        if do_print and i % 10 == 0:
            tmp_avg = score / 2 / (i + 1)
            print(f"{i=} {tmp_avg:.2f}")

    return score / 2 / num_games


def mini_max_vs_random() -> None:
    params = MazeParams(width=5, height=5, end_turn=10)
    mini_max_action_f = lambda state: mini_max_action(state, params.end_turn)
    action_wb = (mini_max_action_f, random_action)
    score = average_score(100, action_wb, params)
    print(f"score: {score:.2f}")


def alpha_beta_vs_mini_max() -> None:
    params = MazeParams(width=5, height=5, end_turn=10)
    mini_max_action_f = lambda state: mini_max_action(state, params.end_turn)
    alpha_beta_action_f = lambda state: alpha_beta_action(state, params.end_turn)
    action_wb = (alpha_beta_action_f, mini_max_action_f)
    score = average_score(100, action_wb, params)
    print(f"score: {score:.2f}")


def run(game="alpha_beta_vs_mini_max") -> None:
    print(f"play {game}")
    if game == "alpha_beta_vs_mini_max":
        return alpha_beta_vs_mini_max()
    if game == "mini_max_vs_random":
        return mini_max_vs_random()
    else:
        raise ValueError(f"Unknown game: {game}")


if __name__ == "__main__":
    fire.Fire(run)
