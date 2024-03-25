import numpy as np
import pytest

from thunder_book.ch06.duct import (
    Node,
    duct_action,
    duct_vs_mcts,
    duct_vs_monte_carlo,
    make_duct_f,
)
from thunder_book.ch06.maze_state import MazeParams, SimulataneousMazeState
from thunder_book.ch06.mcts import MCTSParams


@pytest.fixture
def state() -> SimulataneousMazeState:
    params = MazeParams(height=3, width=3, end_turn=4)
    return SimulataneousMazeState(seed=0, params=params)


@pytest.fixture
def node(state) -> Node:
    return Node(state, MCTSParams(c=1.0, expand_threshold=2))


def test_duct_action(state):
    actual = duct_action(state, 0, 100)
    assert actual == 2

    actual = duct_action(state, 1, 100)
    assert actual == 1


def test_make_duct_f(state):
    assert make_duct_f(100)(state, 0) == 2


def test_duct_vs_mcts():
    duct_vs_mcts(1, 1)


def test_duct_vs_monte_carlo():
    duct_vs_monte_carlo(1, 1)


def test_increment(node):
    node._increment(0.4)
    assert node.n == 1
    assert node.w == 0.4


def test_state_str(state):
    actual = str(state)
    expected = """\
turn: 0
score(A): 0 y: 1 x: 0
score(B): 0 y: 1 x: 2

.6.
A8B
747
"""
    assert actual == expected


def test_expand(node):
    node.expand()
    assert isinstance(node.child_nodeses, np.ndarray)
    assert node.child_nodeses.shape == (3, 3)
    assert node.child_nodeses[0, 0].state.turn == 1
    assert node.child_nodeses[0, 0].state.characters[0].x == 1
    assert node.child_nodeses[0, 0].state.characters[1].x == 1


def test_explore(node):
    value = node.explore()
    assert value == 1.0
    assert node.n == 1
    assert node.w == 1.0


def test_next_child_node(node):
    node.expand()
    child = node.next_child_node()
    assert child.state.turn == 1
    assert child.n == 0


def test_t(node):
    node.explore()
    node.explore()
    node.explore()
    node.explore()
    node.explore()
    actual = node.t
    assert actual == 3


def test_ucb1(node):
    node.explore()
    node.explore()
    node.explore()
    node.explore()
    node.explore()
    actual = node.ucb1(1, 1)
    assert isinstance(actual, float)


def test_action0(node):
    for _ in range(9):
        node.explore()
    assert node.action0() == 0


def test_action1(node):
    for _ in range(9):
        node.explore()
    assert node.action1() == 1


def test_best_i(node):
    for _ in range(9):
        node.explore()
    assert node.best_i() == 0


def test_best_j(node):
    for _ in range(9):
        node.explore()
    assert node.best_j() == 0
