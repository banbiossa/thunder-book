import pytest

from thunder_book.ch05.iterative_deepening import (
    compare_iterative_deepening,
    iterative_deepening_action,
)
from thunder_book.ch05.maze_state import (
    AlternateMazeState,
    MazeParams,
)


@pytest.fixture
def state() -> AlternateMazeState:
    params = MazeParams(width=5, height=5, end_turn=10)
    return AlternateMazeState(0, params)


def test_iterative_deepening(state):
    actual = iterative_deepening_action(state, 10)
    assert actual == 0


def test_compare_iterative_deepening():
    compare_iterative_deepening(1, 1, 1)
