# running stuff
import logging
from typing import Type

import fire
from pydantic import BaseModel

from thunder_book.ch05.maze_state import MCTSParams
from thunder_book.ch05.monte_carlo_tree_search import (
    mcts_action,
    mcts_action_with_time_threshold,
)
from thunder_book.ch08.bitset import ConnectFourBitset
from thunder_book.ch08.game import play_black_and_white
from thunder_book.ch08.maze_state import (
    ActionFunc,
    ConnectFourState,
    MazeParams,
    MazeState,
)
from thunder_book.ch08.random_action import random_action
from thunder_book.util import setup_logging


class ActionStates(BaseModel):
    name: str
    state_types: tuple[Type[MazeState], Type[MazeState]]
    actions: tuple[ActionFunc, ActionFunc]


def compare():
    def mcts_action_f(state):
        return mcts_action(state, 100, MCTSParams(c=1.0, expand_threshold=10))

    def mcts_timed_f(state):
        return mcts_action_with_time_threshold(
            state, 1, mcst_params=MCTSParams(c=1.0, expand_threshold=10)
        )

    action_funcs = [
        ActionStates(
            name="mcts_vs_random",
            state_types=(ConnectFourState, ConnectFourState),
            actions=(mcts_action_f, random_action),
        ),
        ActionStates(
            name="mcts_time_vs_random",
            state_types=(ConnectFourState, ConnectFourState),
            actions=(mcts_timed_f, random_action),
        ),
        ActionStates(
            name="mcts_time_vs_mcts_time",
            state_types=(ConnectFourState, ConnectFourState),
            actions=(mcts_timed_f, mcts_timed_f),
        ),
        ActionStates(
            name="random_vs_random",
            state_types=(ConnectFourState, ConnectFourState),
            actions=(random_action, random_action),
        ),
        ActionStates(
            name="bitset_vs_mcts",
            state_types=(ConnectFourBitset, ConnectFourState),
            actions=(mcts_timed_f, mcts_timed_f),
        ),
        ActionStates(
            name="bitset_vs_random",
            state_types=(ConnectFourBitset, ConnectFourState),
            actions=(mcts_timed_f, random_action),
        ),
    ]

    logger = logging.getLogger("file_logger")

    logger.info("| name | score |")
    logger.info("| ---- | ----- |")
    for action_func in action_funcs:
        logger.debug(f"{action_func.name}")
        score = play_black_and_white(
            MazeParams(width=7, height=6),
            action_func.actions,
            state_types=action_func.state_types,
            num_games=100,
            print_every=10,
        )
        logger.info(f"| {action_func.name} | {score:.2f} |")


if __name__ == "__main__":
    setup_logging()
    fire.Fire(compare)
