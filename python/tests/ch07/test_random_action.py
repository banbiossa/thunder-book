import pytest

from thunder_book.ch07.maze_state import (
    MazeParams,
    WallMazeState,
)
from thunder_book.ch07.random_action import random_action


@pytest.fixture
def state() -> WallMazeState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return WallMazeState(0, params)


def test_random_action(state):
    actual = random_action(state)
    assert actual in [0, 2]
