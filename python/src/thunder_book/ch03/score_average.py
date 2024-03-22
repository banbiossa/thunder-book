import logging
import random
from datetime import datetime

import fire
import numpy as np
from pydantic import BaseModel
from tqdm import tqdm

from thunder_book.ch03.beam_search import (
    beam_search_action,
    beam_search_action_with_time_threshold,
)
from thunder_book.ch03.chokudai_search import chokudai_search_action_with_time_threshold
from thunder_book.ch03.maze_state import (
    ActionFunc,
    MazeParams,
    MazeState,
    greedy_action,
    random_action,
)
from thunder_book.util import setup_logging

logger = logging.getLogger(__name__)


def test_score(
    game_number: int,
    action_func: ActionFunc,
    params: MazeParams,
) -> tuple[float, float]:
    start = datetime.now()
    random.seed(0)
    scores = []
    for i in tqdm(range(game_number)):
        state = MazeState(random.randint(0, game_number ^ 2), params)
        while not state.is_done():
            state.advance(action_func(state))
        scores.append(state.game_score)

    scores = np.array(scores)
    elapsed = (datetime.now() - start).total_seconds()
    return scores.mean(), elapsed


class NameAction(BaseModel):
    name: str
    action: ActionFunc


def test_multiple(num_games=100):
    params = MazeParams(height=30, width=30, end_turn=100)

    # refactor the above test_score calls into NameAction instances
    name_actions = [
        NameAction(name="random", action=random_action),
        NameAction(name="greedy", action=greedy_action),
        NameAction(
            name="beam_search_width_2",
            action=lambda x: beam_search_action(x, beam_width=2, beam_depth=params.end_turn),
        ),
        NameAction(
            name="beam_search_width_5_with_1ms",
            action=lambda x: beam_search_action_with_time_threshold(
                x, beam_width=5, time_threshold=1
            ),
        ),
        NameAction(
            name="beam_search_width_5_with_10ms",
            action=lambda x: beam_search_action_with_time_threshold(
                x, beam_width=5, time_threshold=10
            ),
        ),
        NameAction(
            name="chokudai_search_width_1_with_1ms",
            action=lambda x: chokudai_search_action_with_time_threshold(
                x, beam_width=1, beam_depth=params.end_turn, time_threshold=1
            ),
        ),
        NameAction(
            name="chokudai_search_width_1_with_10ms",
            action=lambda x: chokudai_search_action_with_time_threshold(
                x, beam_width=1, beam_depth=params.end_turn, time_threshold=10
            ),
        ),
    ]

    logger = logging.getLogger("file_logger")
    logger.info("| name | score | time |")
    logger.info("| ------ | ----- | ---- |")
    for name_action in name_actions:
        score, time = test_score(num_games, name_action.action, params)
        logger.info(f"| {name_action.name} | {score:.0f} | {time:.2f} |")


if __name__ == "__main__":
    # logging.basicConfig(level=logging.INFO)
    # logger = logging.getLogger()
    # logger.setLevel(logging.WARNING)
    setup_logging()
    fire.Fire(test_multiple)
