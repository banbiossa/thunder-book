import pytest
from bitarray import bitarray

from thunder_book.ch07.maze_state import (
    MazeParams,
    WallMazeState,
)
from thunder_book.ch07.multibit import Mat, MultibitState

(
    Mat,
    MultibitState,
    bitarray,
)


@pytest.fixture
def state() -> WallMazeState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return WallMazeState(0, params)


@pytest.fixture
def mat() -> Mat:
    return Mat(MazeParams(height=3, width=5, end_turn=4))


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


def test_mat(mat):
    assert mat[0, 0] == 0
    mat[0, 0] = 1
    assert mat[0, 0] == 1


def test_up(mat):
    mat[1, 1] = 1
    up = mat.up()
    assert up[0, 1] == 1


def test_down(mat):
    mat[0, 0] = 1
    down = mat.down()
    assert down[1, 0] == 1
