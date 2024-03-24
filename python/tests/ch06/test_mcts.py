import pytest

from thunder_book.ch06.maze_state import MazeParams, SimulataneousMazeState
from thunder_book.ch06.mcts import EvenNode, MCTSParams, OddNode, Playout


@pytest.fixture
def state() -> SimulataneousMazeState:
    params = MazeParams(height=5, width=5, end_turn=10)
    return SimulataneousMazeState(seed=0, params=params)


@pytest.fixture
def node(state) -> EvenNode:
    return EvenNode(state, MCTSParams(c=1.0, expand_threshold=2), is_root=True)


def test_print_state(state):
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


def test_playout(state):
    actual = Playout(state).playout()
    assert actual == 0.5


def test_even_node_increment(node):
    node._increment(0.5)
    assert node.w == 0.5
    assert node.n == 1


def test_even_node_expand(node):
    node.expand()
    assert isinstance(node.child_nodes[0], OddNode)


def test_str(node):
    actual = str(node)
    expected = """\
EvenNode(w=0, n=0) <<<
"""
    assert actual == expected

    node.explore()
    actual = str(node.child_nodes[0])
    expected = """\
EvenNode(w=0.5, n=1)
__ OddNode(w=0.5, n=1) <<<
__ OddNode(w=0, n=0)
__ OddNode(w=0, n=0)
__ OddNode(w=0, n=0)
"""
    assert actual == expected


def test_explore(node):
    node.explore()
    node.explore()
    node.explore()
    node.explore()
    node.explore()

    actual = str(node)
    expected = """\
EvenNode(w=1.5, n=5) <<<
__ OddNode(w=0.5, n=1)
__ OddNode(w=1.0, n=1)
__ OddNode(w=1.0, n=2)
__ __ EvenNode(w=0, n=0)
__ __ EvenNode(w=0, n=0)
__ __ EvenNode(w=0, n=0)
__ __ EvenNode(w=0, n=0)
__ OddNode(w=1.0, n=1)
"""
    assert actual == expected


def test_next_child_node(node):
    node.explore()
    actual = node.next_child_node()
    assert isinstance(actual, OddNode)
