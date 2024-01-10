import logging
import random
from datetime import datetime
from typing import Callable

import fire
import numpy as np
from tqdm import tqdm

from thunder_book.ch03 import constants
from thunder_book.ch03.beam_search import (
    beam_search_action,
    beam_search_action_with_time_threshold,
)
from thunder_book.ch03.chokudai_search import chokudai_search_action_with_time_threshold
from thunder_book.ch03.maze_state import MazeState, greey_action, random_action

logger = logging.getLogger(__name__)


def test_score(game_number: int, action_func: Callable):
    logger.info(f"Play {game_number} games with {action_func=}")
    start = datetime.now()
    random.seed(0)
    scores = []
    for i in tqdm(range(game_number)):
        logger.info(f"game {i}")
        state = MazeState(random.randint(0, game_number ^ 2))
        while not state.is_done():
            logger.debug("action")
            state.advance(action_func(state))
        logger.info(f"game {i} done")
        scores.append(state.game_score)

    scores = np.array(scores)
    elapsed = (datetime.now() - start).total_seconds()
    print(f"Score:\t{scores.mean():.0f}, time:\t{elapsed:.2f}")


def test_multiple(games=100):
    print(f"play {games=}")

    print("random")
    test_score(games, random_action)

    print("greedy")
    test_score(games, greey_action)

    print("beam_search width 2")
    test_score(
        games,
        lambda x: beam_search_action(x, beam_width=2, beam_depth=constants.END_TURN),
    )

    print("beam_search width 5 with 1ms")
    test_score(
        games,
        lambda x: beam_search_action_with_time_threshold(
            x, beam_width=5, time_threshold=1
        ),
    )

    print("beam_search width 5 with 10ms")
    test_score(
        games,
        lambda x: beam_search_action_with_time_threshold(
            x, beam_width=5, time_threshold=10
        ),
    )

    print("chokudai search width 1 with 1ms")
    test_score(
        games,
        lambda x: chokudai_search_action_with_time_threshold(
            x,
            beam_width=1,
            beam_depth=constants.END_TURN,
            time_threshold=1,
        ),
    )

    print("chokudai search width 1 with 10ms")
    test_score(
        games,
        lambda x: chokudai_search_action_with_time_threshold(
            x,
            beam_width=1,
            beam_depth=constants.END_TURN,
            time_threshold=10,
        ),
    )

    print("chokudai search width 1 with 100ms")
    test_score(
        games,
        lambda x: chokudai_search_action_with_time_threshold(
            x,
            beam_width=1,
            beam_depth=constants.END_TURN,
            time_threshold=100,
        ),
    )


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    logger = logging.getLogger()
    logger.setLevel(logging.WARNING)
    fire.Fire(test_multiple)
