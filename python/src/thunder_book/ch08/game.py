import logging
from typing import Type

from thunder_book.ch08.maze_state import (
    ActionFunc,
    MazeParams,
    MazeState,
)
from thunder_book.util import printerr

logger = logging.getLogger(__name__)


def play_game(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    state_types: tuple[Type[MazeState], Type[MazeState]],
    should_print: bool = False,
) -> float:
    # state = ConnectFourState(params)
    states = [state_types[0](params), state_types[1](params)]

    if should_print:
        logger.debug(states[0].to_string())

    player = 0
    while not states[0].is_done():
        action = actions[player](states[player])
        for state in states:
            state.advance(action)
        player ^= 1
        if should_print:
            logger.debug(states[0].to_string())
            printerr(states[0].to_string())

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
            logger.debug(f"{i+1} {total_score / (i+1):.2f}")
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
