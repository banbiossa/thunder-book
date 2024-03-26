import pytest

from thunder_book.ch07.maze_state import (
    MazeParams,
)
from thunder_book.ch07.numpy_state import NumpyState


@pytest.fixture
def state() -> NumpyState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return NumpyState(0, params)


def test_str(state):
    actual = str(state)
    expected = """\
turn: 0
score: 0

@5.#3
3###7
93524
"""

    assert actual == expected


def test_get_distance_to_nearest_point(state):
    assert state.get_distance_to_nearest_point() == 1
