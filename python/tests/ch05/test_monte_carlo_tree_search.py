import pytest

from thunder_book.ch05.maze_state import (
    AlternateMazeState,
    MazeParams,
    MCTSParams,
)
from thunder_book.ch05.monte_carlo_tree_search import Node


@pytest.fixture
def state() -> AlternateMazeState:
    params = MazeParams(width=3, height=3, end_turn=4)
    return AlternateMazeState(0, params)


@pytest.fixture
def node(state) -> Node:
    mcts_params = MCTSParams(c=1.0, expand_threshold=2)
    return Node(state, mcts_params, is_root=True)


def test_expand(node):
    node.expand()

    # state of child node should be +1 from parent node
    for child_node in node.child_nodes:
        assert child_node.state.turn == node.state.turn + 1


def test_make_node(node):
    assert node.n == 0


def test_increment(node):
    node._increment(1.0)
    assert node.n == 1
    assert node.w == 1.0


def test_ucb1(node):
    node.w = 1.0
    node.n = 3.0
    actual = node.ucb1(10)
    assert actual > 0


def test_evaluate(node):
    assert not node.child_nodes
    assert node.n == 0
    node.evaluate()
    assert not node.child_nodes
    assert node.n == 1


def test_to_string(node):
    actual = str(node.state)
    expected = """\
turn: 0
score(A): 0 y: 1 x: 0
score(B): 0 y: 1 x: 2

66.
A4B
876
"""
    assert actual == expected


def test_next_child_node(node):
    node.expand()
    actual = node.next_child_node()
    assert actual.n == 0
    assert actual.action == 0


def test_str(node):
    actual = str(node)
    expected = "0(0) <<<\n"
    assert actual == expected

    node.expand()
    actual = str(node)
    expected = """\
0(0) <<<
__ 0=>0(0)
__ 2=>0(0)
__ 3=>0(0)
"""
    assert actual == expected

    child_node = node.next_child_node()
    actual = str(child_node)
    expected = """\
0(0)
__ 0=>0(0) <<<
__ 2=>0(0)
__ 3=>0(0)
"""
    assert actual == expected


def test_evaluate_more(node):
    node.evaluate()
    node.evaluate()
    node.evaluate()
    node.evaluate()
    child_node = node.next_child_node()
    actual = str(child_node)
    expected = """\
4(3.0)
__ 0=>1(0.0)
__ 2=>1(0.0)
__ 3=>0(0) <<<
"""
    assert actual == expected
    assert not child_node.child_nodes

    node.evaluate()
    node.evaluate()
    child_node = node.next_child_node()
    actual = str(child_node)
    expected = """\
6(5.0)
__ 0=>2(0.0)
__ __ 1=>0(0)
__ __ 2=>0(0)
__ __ 3=>0(0)
__ 2=>1(0.0) <<<
__ 3=>1(0.0)
"""
    assert actual == expected
    assert not child_node.child_nodes
