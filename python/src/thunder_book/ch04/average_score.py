import logging
import random
from typing import Callable

import fire
from tqdm import tqdm

from thunder_book.ch04.auto_move_maze_state import MazeParams
from thunder_book.ch04.auto_move_maze_state import MazeState as State
from thunder_book.ch04.hill_climb import hill_climb
from thunder_book.ch04.random_action import random_action
from thunder_book.ch04.simulated_annealing import simulated_annealing
from thunder_book.util import setup_logging

logger = logging.getLogger(__name__)


def run_many(
    action_func: Callable[[State], State],
    num_games: int,
):
    score_mean = 0
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    for _ in tqdm(range(num_games)):
        state = State(random.randint(0, num_games ^ 2), params)
        last_state = action_func(state)
        score = last_state.get_score()
        score_mean += score
    score_mean /= num_games

    return score_mean


type GAMES_TYPE = tuple[str, Callable[[State], State]]


def run(num_simulate=10000, num_games=100):
    games: list[GAMES_TYPE] = [
        ("random_action", random_action),
        ("hill_climb", lambda state: hill_climb(state, num_simulate)),
        (
            "simulated_annealing",
            lambda state: simulated_annealing(
                state,
                num_simulate,
                start_temp=500,
                end_temp=10,
            ),
        ),
    ]

    # print what to play
    logger.debug(
        f"average of {num_games} games for {[name for name, _ in games]} in {num_simulate} simulations"
    )

    file_logger = logging.getLogger("file_logger")
    file_logger.info("|name|score|")
    file_logger.info("|---|----|")
    # play
    for name, action_func in games:
        logger.debug(f"play {name}")
        score = run_many(action_func, num_games)
        file_logger.info(f"|{name}|{score:.2f}|")


if __name__ == "__main__":
    setup_logging()
    fire.Fire(run)
