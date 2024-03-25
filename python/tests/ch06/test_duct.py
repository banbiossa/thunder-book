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

Node, duct_action, make_duct_f, duct_vs_mcts, duct_vs_monte_carlo


@pytest.fixture
def state() -> SimulataneousMazeState:
    params = MazeParams(height=5, width=5, end_turn=10)
    return SimulataneousMazeState(seed=0, params=params)


@pytest.fixture
def node(state) -> Node:
    return Node(state, MCTSParams(c=1.0, expand_threshold=10))


def test_increment(node):
    node._increment(0.4)
    assert node.n == 1
    assert node.w == 0.4


def test_state_str(state):
    actual = str(state)
    expected = """\
turn: 0
score(A): 0 y: 2 x: 1
score(B): 0 y: 2 x: 3

84.48
57475
4A8B4
84948
11411
"""
    assert actual == expected


def test_expand(node):
    node.expand()
    assert isinstance(node.child_nodeses, np.ndarray)
    assert node.child_nodeses.shape == (4, 4)
    assert node.child_nodeses[0, 0].state.turn == 1
    assert node.child_nodeses[0, 0].state.characters[0].x == 2
    assert node.child_nodeses[0, 0].state.characters[1].x == 4
