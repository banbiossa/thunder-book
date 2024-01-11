from typing import Callable
import random

from thunder_book.ch04.auto_move_maze_state import MazeState as State
from thunder_book.ch04 import constants


def random_action(state: State) -> State:
    now_state = state.copy()
    for character_id in range(constants.CHARACTER_N):
        y = random.randint(0, constants.H - 1)
        x = random.randint(0, constants.W - 1)
        now_state.set_character(character_id, y, x)
    return now_state


StringAIPair = tuple[str, Callable[[State], State]]


def play_game(name: str, action_func: Callable[[State], State], seed: int):
    state = State(seed)
    end_state = action_func(state)
    score = end_state.get_score()
    print(f"Score of {name} is {score}")


if __name__ == "__main__":
    play_game("random", random_action, 0)
