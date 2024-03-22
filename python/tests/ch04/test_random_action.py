from thunder_book.ch04.auto_move_maze_state import MazeParams, MazeState
from thunder_book.ch04.random_action import random_action


def test_random_action():
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    state = MazeState(0, params)
    actual = random_action(state)
    assert isinstance(actual, MazeState)
