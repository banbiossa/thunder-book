import pytest

from thunder_book.ch05.maze_state import (
    AlternateMazeState,
    MazeParams,
)
from thunder_book.ch05.thunder_search import (
    TNode,
    thunder_search_action,
    thunder_search_action_with_time_threshold,
    thunder_search_vs_mcts,
    thunder_vs_iterative_deepening_timebound,
    thunder_vs_mcts_timebound,
)

(
    thunder_search_action,
    thunder_search_action_with_time_threshold,
    thunder_search_vs_mcts,
    thunder_vs_iterative_deepening_timebound,
    thunder_vs_mcts_timebound,
    TNode,
)


@pytest.fixture
def state() -> AlternateMazeState:
    params = MazeParams(width=3, height=3, end_turn=4)
    return AlternateMazeState(0, params)


@pytest.fixture
def node(state) -> TNode:
    return TNode(state=state)


def test_increment(node):
    node._increment(1)
    assert node.w == 1
    assert node.n == 1


def test_print_state(node):
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


def test_expand(node):
    node.expand()
    assert len(node.child_nodes) == len(node.state.legal_actions())
    for child_node in node.child_nodes:
        assert child_node.state.turn == node.state.turn + 1


def test_next_child_node(node):
    node.expand()
    child = node.next_child_node()
    assert child.n == 0


def test_evaluate(node):
    assert node.evaluate() == 0.5
    node.expand()
    node.evaluate()
    assert node.n == 2
    assert node.w == 1.5


def test_print_tree(node):
    actual = str(node)
    expected = """\
0(0.0)
"""
    assert actual == expected

    node.evaluate()
    actual = str(node)
    expected = """\
1(0.5)
__ 0=>0(0.0)
__ 2=>0(0.0)
__ 3=>0(0.0)
"""
    assert actual == expected

    node.evaluate()
    actual = str(node)
    expected = """\
2(1.5)
__ 0=>1(0.0)
__ __ 1=>0(0.0)
__ __ 2=>0(0.0)
__ __ 3=>0(0.0)
__ 2=>0(0.0)
__ 3=>0(0.0)
"""
    assert actual == expected

    node.evaluate()
    actual = str(node)
    expected = """\
3(2.5)
__ 0=>1(0.0)
__ __ 1=>0(0.0)
__ __ 2=>0(0.0)
__ __ 3=>0(0.0)
__ 2=>1(0.0)
__ __ 1=>0(0.0)
__ __ 2=>0(0.0)
__ __ 3=>0(0.0)
__ 3=>0(0.0)
"""
    assert actual == expected


def test_best_child_node(node):
    node.evaluate()
    node.evaluate()
    node.evaluate()
    node.evaluate()
    node.evaluate()
    actual = str(node)
    expected = """\
5(4.5)
__ 0=>2(0.0)
__ __ 1=>1(1.0)
__ __ __ 0=>0(0.0)
__ __ __ 1=>0(0.0)
__ __ __ 2=>0(0.0)
__ __ __ 3=>0(0.0)
__ __ 2=>0(0.0)
__ __ 3=>0(0.0)
__ 2=>1(0.0)
__ __ 1=>0(0.0)
__ __ 2=>0(0.0)
__ __ 3=>0(0.0)
__ 3=>1(0.0)
__ __ 1=>0(0.0)
__ __ 2=>0(0.0)
__ __ 3=>0(0.0)
"""
    assert actual == expected
    assert node.best_action() == 0
