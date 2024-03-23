import pytest

from thunder_book.ch05.alpha_beta import TimeKeeper, alpha_beta_action, alpha_beta_score
from thunder_book.ch05.maze_state import AlternateMazeState, MazeParams


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


def test_alpha_beta_score(state):
    # time 0
    time_keeper = TimeKeeper(0)
    assert alpha_beta_score(state, -100, 100, 10, time_keeper) == 0

    # depth 0
    time_keeper = TimeKeeper(100)
    assert alpha_beta_score(state, -100, 100, 0, time_keeper) == 0

    time_keeper = TimeKeeper(10000)
    assert alpha_beta_score(state, -100, 100, 1, time_keeper) == 9

    time_keeper = TimeKeeper(10000)
    assert alpha_beta_score(state, -100, 100, 2, time_keeper) == 1


def test_alpha_beta_action(state):
    actual = alpha_beta_action(state, 0)
    assert actual == 1

    actual = alpha_beta_action(state, 1)
    assert actual == 1

    actual = alpha_beta_action(state, 2)
    assert actual == 1
