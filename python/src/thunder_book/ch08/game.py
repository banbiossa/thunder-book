from typing import Type

import fire

from thunder_book.ch05.monte_carlo_tree_search import (
    mcts_action,
    mcts_action_with_time_threshold,
)
from thunder_book.ch08.bitset import ConnectFourBitset
from thunder_book.ch08.maze_state import (
    ActionFunc,
    ConnectFourState,
    MazeParams,
    MazeState,
)
from thunder_book.ch08.random_action import random_action


def play_game(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    state_types: tuple[Type[MazeState], Type[MazeState]],
    should_print: bool = False,
) -> float:
    # state = ConnectFourState(params)
    states = [state_types[0](params), state_types[1](params)]

    if should_print:
        print(states[0].to_string())

    player = 0
    while not states[0].is_done():
        action = actions[player](states[player])
        for state in states:
            state.advance(action)
        player ^= 1
        if should_print:
            print(states[0].to_string())

    return state.white_score()


def play_many(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    state_types: tuple[Type[MazeState], Type[MazeState]],
    num_games: int,
    print_every: int = 10,
) -> float:
    total_score = 0.0
    for i in range(num_games):
        total_score += play_game(params, actions, state_types)

        if print_every > 0 and (i + 1) % print_every == 0:
            print(f"{i+1} {total_score / (i+1):.2f}")
    return total_score / num_games


def play_black_and_white(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    state_types: tuple[Type[MazeState], Type[MazeState]],
    num_games: int,
    print_every: int = 10,
) -> float:
    total = play_many(params, actions, state_types, num_games, print_every)
    actions_bw = (actions[1], actions[0])
    total += 1 - play_many(params, actions_bw, state_types, num_games, print_every)
    return total / 2


# running stuff


def run_connect_four():
    mcts_action_f = lambda state: mcts_action(state, 100)
    score = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_action_f, random_action),
        state_types=(ConnectFourState, ConnectFourState),
        num_games=100,
    )
    print(score)


def run_connect_four_time():
    mcts_timed_f = lambda state: mcts_action_with_time_threshold(state, 1)
    score = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_timed_f, random_action),
        state_types=(ConnectFourState, ConnectFourState),
        num_games=100,
    )
    print(score)


def run_time_on_time():
    mcts_timed_f = lambda state: mcts_action_with_time_threshold(state, 1)
    score = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_timed_f, mcts_timed_f),
        state_types=(ConnectFourState, ConnectFourState),
        num_games=100,
    )
    print(score)


def run_random():
    mcts_timed_f = lambda state: mcts_action_with_time_threshold(state, 1)
    score = play_black_and_white(
        MazeParams(width=7, height=6),
        (random_action, random_action),
        state_types=(ConnectFourState, ConnectFourState),
        num_games=100,
    )
    print(score)


def run_bitstate_vs_mcts():
    mcts_timed_f = lambda state: mcts_action_with_time_threshold(state, 10)
    score = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_timed_f, mcts_timed_f),
        state_types=(ConnectFourBitset, ConnectFourState),
        num_games=100,
    )
    print(score)


def run_bitset_vs_random():
    mcts_timed_f = lambda state: mcts_action_with_time_threshold(state, 1)
    score = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_timed_f, random_action),
        state_types=(ConnectFourBitset, ConnectFourState),
        num_games=100,
    )
    print(score)


def main(name="bitset_vs_mcts"):
    print(name)
    if name == "connect_four":
        run_connect_four()
        return
    if name == "timed":
        run_connect_four_time()
        return
    if name == "time_vs_time":
        run_time_on_time()
    if name == "random":
        run_random()
    if name == "bitset_vs_mcts":
        run_bitstate_vs_mcts()
        return
    if name == "bitset_vs_random":
        run_bitset_vs_random()


if __name__ == "__main__":
    fire.Fire(main)
