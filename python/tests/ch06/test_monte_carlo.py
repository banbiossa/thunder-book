import pytest

from thunder_book.ch06.maze_state import MazeParams, SimulataneousMazeState
from thunder_book.ch06.monte_carlo import (
    Playout,
    make_monte_carlo_f,
    monte_carlo_action,
    monte_carlo_vs_random,
)

Playout, monte_carlo_action, make_monte_carlo_f, monte_carlo_vs_random


@pytest.fixture
def state() -> SimulataneousMazeState:
    params = MazeParams(height=5, width=5, end_turn=10)
    return SimulataneousMazeState(seed=0, params=params)


def test_playout(state):
    actual = Playout(state).playout(0)
    assert actual == 0.5


def test_monte_carlo_action(state):
    actual = monte_carlo_action(state, 0, 100)
    assert actual == 0


def test_make_monte_carlo_f(state):
    actual = make_monte_carlo_f(100)(state, 0)
    assert actual == 0


def test_monte_carlo_vs_random():
    monte_carlo_vs_random(1, 1)
