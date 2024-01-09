import random
from typing import Callable
import logging

import numpy as np
import fire

from thunder_book.ch03.maze_state import MazeState, random_action, greey_action
from thunder_book.ch03.beam_search import beam_search_action
from thunder_book.ch03 import constants

logger = logging.getLogger(__name__)


def test_score(game_number: int, action_func: Callable):
    logger.info(f"Play {game_number} games with {action_func=}")
    random.seed(0)
    scores = []
    for i in range(game_number):
        logger.info(f"game {i}")
        state = MazeState(random.randint(0, game_number ^ 2))
        while not state.is_done():
            logger.debug("action")
            state.advance(action_func(state))
        logger.info(f"game {i} done")
        scores.append(state.game_score)

    scores = np.array(scores)
    print(f"Score:\t{scores.mean()}")


def test_multiple(games=100):
    print(f"play {games=}")

    print("random")
    test_score(games, random_action)

    print("greedy")
    test_score(games, greey_action)

    print("beam_search")
    test_score(
        games,
        lambda x: beam_search_action(x, beam_width=2, beam_depth=constants.END_TURN),
    )


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    logger = logging.getLogger()
    logger.setLevel(logging.WARNING)
    fire.Fire(test_multiple)
