import pytest

from thunder_book.ch06.maze_state import MazeParams, SimulataneousMazeState
from thunder_book.ch06.random_action import random_action


@pytest.fixture
def state() -> SimulataneousMazeState:
    params = MazeParams(height=5, width=5, end_turn=10)
    return SimulataneousMazeState(seed=0, params=params)


def test_random_action(state):
    actual = random_action(state, 0)
    assert actual in [0, 1, 2, 3]
