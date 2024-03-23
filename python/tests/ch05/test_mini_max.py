import pytest

from thunder_book.ch05.maze_state import AlternateMazeState, MazeParams
from thunder_book.ch05.mini_max import mini_max_action, mini_max_score


@pytest.fixture
def state() -> AlternateMazeState:
    params = MazeParams(width=5, height=5, end_turn=10)
    return AlternateMazeState(0, params)


# just to check the state
def test_to_string(state):
    actual = str(state)
    expected = """\
turn: 0
score(A): 0 y: 2 x: 1
score(B): 0 y: 2 x: 3

66.48
76475
9A3B8
24219
48924
"""
    assert actual == expected


def test_mini_max_score(state):
    actual = mini_max_score(state, 0)
    assert actual == 0

    actual = mini_max_score(state, 1)
    assert actual == 9

    actual = mini_max_score(state, 2)
    assert actual == 1


def test_mini_max_action(state):
    actual = mini_max_action(state, 0)
    assert actual == 1

    actual = mini_max_action(state, 1)
    assert actual == 1

    actual = mini_max_action(state, 2)
    assert actual == 1
