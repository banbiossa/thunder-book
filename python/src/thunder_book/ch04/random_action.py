import random

from thunder_book.ch04.auto_move_maze_state import MazeState as State
from thunder_book.ch04.auto_move_maze_state import play_game


def random_action(state: State) -> State:
    now_state = state.copy()
    for character_id in range(state.params.num_characters):
        y = random.randint(0, state.params.height - 1)
        x = random.randint(0, state.params.width - 1)
        now_state.set_character(character_id, y, x)
    return now_state


if __name__ == "__main__":
    play_game("random", random_action, 42)
