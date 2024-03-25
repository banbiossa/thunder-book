import pytest

from thunder_book.ch07.maze_state import (
    MazeParams,
)
from thunder_book.ch07.multibit import Mat, MultibitState


@pytest.fixture
def state() -> MultibitState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return MultibitState(0, params)


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


def test_left(mat):
    mat[0, 1] = 1
    left = mat.left()
    assert left[0, 2] == 1


def test_right(mat):
    mat[0, 1] = 1
    right = mat.right()
    assert right[0, 0] == 1


def test_expand(mat):
    mat[1, 1] = 1
    mat.expand()
    assert mat[0, 1] == 1
    assert mat[2, 1] == 1
    assert mat[1, 2] == 1
    assert mat[1, 0] == 1


def test_andeq_not(mat):
    mat[1, 1] = 1
    mat.andeq_not(mat)
    assert mat[1, 1] == 0


def test_is_any_equal(mat):
    assert not mat.is_any_equal(mat)
    mat[1, 1] = 1
    assert mat.is_any_equal(mat)


def test_state_get_distance(state):
    assert state.get_distance_to_nearest_point() == 1


def test_state_points(state):
    assert state.character.on(0, 0)
    assert state.points[0, 0] == 0
