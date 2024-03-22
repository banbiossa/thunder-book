import random
from typing import Callable

import fire
from tqdm import tqdm

from thunder_book.ch04.auto_move_maze_state import MazeParams
from thunder_book.ch04.auto_move_maze_state import MazeState as State
from thunder_book.ch04.hill_climb import hill_climb
from thunder_book.ch04.random_action import random_action
from thunder_book.ch04.simulated_annealing import simulated_annealing


def test_ai_score(
    name: str,
    action_func: Callable[[State], State],
    game_number: int,
):
    score_mean = 0
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    for _ in tqdm(range(game_number)):
        state = State(random.randint(0, 100000), params)
        last_state = action_func(state)
        score = last_state.get_score()
        score_mean += score
    score_mean /= game_number
    print(f"score of {name}: {score_mean:.2f}")


type GAMES_TYPE = tuple[str, Callable[[State], State]]


def run(simulate_number=10000, game_number=100):
    games: list[GAMES_TYPE] = [
        ("random_action", random_action),
        ("hill_climb", lambda state: hill_climb(state, simulate_number)),
        (
            "simulated_annealing",
            lambda state: simulated_annealing(
                state,
                simulate_number,
                start_temp=500,
                end_temp=10,
            ),
        ),
    ]

    # print what to play
    print(
        f"average of {game_number} games for {[name for name, _ in games]} in {simulate_number} simulations"
    )

    # play
    for name, action_func in games:
        print(f"play {name}")
        test_ai_score(name, action_func, game_number)


if __name__ == "__main__":
    fire.Fire(run)
