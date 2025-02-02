import pytest
from bitarray import bitarray

from thunder_book.ch07.maze_state import (
    MazeParams,
)
from thunder_book.ch07.singlebit import SinglebitState, SMat


@pytest.fixture
def state() -> SinglebitState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return SinglebitState(0, params)


@pytest.fixture
def mat() -> SMat:
    return SMat(MazeParams(height=3, width=5, end_turn=4))


def test_init(mat):
    assert mat.bits == bitarray("0" * 15)


def test_left_mask(mat):
    assert mat.left_mask == bitarray("""\
                                     01111\
                                     01111\
                                     01111\
                                     """)


def test_right_mask(mat):
    assert mat.right_mask == bitarray("""\
                                      11110\
                                      11110\
                                      11110\
                                      """)


def test_set(mat):
    mat[1, 2] = 1
    assert mat.bits == bitarray("""\
                                00000\
                                00100\
                                00000\
                                """)


def test_get(mat):
    mat[1, 2] = 1
    assert mat.get(1, 2) == 1


def test_remove(mat):
    mat[1, 2] = 1
    assert mat.get(1, 2) == 1
    mat.remove(1, 2)
    assert mat.get(1, 2) == 0


def test_up(mat):
    mat[1, 2] = 1
    assert mat.up() == bitarray("""\
                                00000\
                                00000\
                                00100\
                                """)


def test_down(mat):
    mat[1, 2] = 1
    assert mat.down() == bitarray("""\
                                00100\
                                00000\
                                00000\
                                """)


def test_left(mat):
    mat[1, 2] = 1
    assert mat.left() == bitarray("""\
                                00000\
                                00010\
                                00000\
                                """)
    mat[1, 4] = 1
    assert mat.bits == bitarray("""\
                                00000\
                                00101\
                                00000\
                                """)
    assert mat.left() == bitarray("""\
                                00000\
                                00010\
                                00000\
                                """)


def test_right(mat):
    mat[1, 2] = 1
    assert mat.right() == bitarray("""\
                                00000\
                                01000\
                                00000\
                                """)
    mat[1, 0] = 1
    assert mat.bits == bitarray("""\
                                00000\
                                10100\
                                00000\
                                """)
    assert mat.right() == bitarray("""\
                                00000\
                                01000\
                                00000\
                                """)


def test_expand(mat):
    mat[1, 2] = 1
    assert mat.bits == bitarray("""\
                                00000\
                                00100\
                                00000\
                                """)
    mat.expand()
    assert mat.bits == bitarray("""\
                                00100\
                                01110\
                                00100\
                                """)


def test_eq(mat):
    copied = mat.copy()
    assert mat == copied


def test_andeq_not(mat):
    wall = mat.copy()
    expected = mat.copy()

    mat[1, 2] = 1
    mat[1, 3] = 1
    wall[1, 3] = 1
    expected[1, 2] = 1

    mat.andeq_not(wall)
    assert mat == expected


def test_is_any_equal(mat):
    assert not mat.is_any_equal(mat)

    mat[1, 2] = 1
    assert mat.is_any_equal(mat)


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
