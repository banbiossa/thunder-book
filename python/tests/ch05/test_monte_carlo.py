import pytest

from thunder_book.ch05.maze_state import AlternateMazeState, MazeParams
from thunder_book.ch05.monte_carlo import (
    Playout,
    compare_monte_carlo,
    play_monte_carlo_vs_random,
    primitive_monte_carlo_action,
)


@pytest.fixture
def state() -> AlternateMazeState:
    params = MazeParams(width=5, height=5, end_turn=10)
    return AlternateMazeState(0, params)


def test_playout(state):
    # ここが怪しい
    score = Playout(state).playout()
    assert score == 0.0


def test_advance(state):
    playout = Playout(state).advance(0)

    assert playout.state.turn == 1
    assert playout.state.characters[0].x == 3
    assert playout.playout() == 0.0

    assert Playout(state).advance(0).playout() == 1.0


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


def test_primitive_monte_carlo_action(state):
    actual = primitive_monte_carlo_action(state, 100)
    assert actual == 1


def test_play_monte_carlo():
    play_monte_carlo_vs_random(num_playout=1)


def test_compare_monte_carlo():
    compare_monte_carlo(a=1, b=2)
