import pytest

from thunder_book.ch04.auto_move_maze_state import Coord, MazeParams, MazeState


@pytest.fixture
def state():
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    t_state = MazeState(1, params)
    t_state.init_characters()
    return t_state


def test_make_state(state):
    assert state.turn == 0
    assert state.points.shape == (5, 5)
    assert state.points.max() <= 9


def test_str(state):
    actual = str(state)
    expected = """\
turn: 0
score: 0

291@@
777@3
17.66
9.743
915..
"""
    assert actual == expected


def test_init_characters(state):
    assert state.chracters[0] == Coord(x=4, y=0)
    assert state.chracters[1] == Coord(x=3, y=0)
    assert state.chracters[2] == Coord(x=3, y=1)


def test_set_character(state):
    assert state.chracters[0] == Coord(x=4, y=0)
    state.set_character(0, 1, 1)
    assert state.chracters[0] == Coord(x=1, y=1)
