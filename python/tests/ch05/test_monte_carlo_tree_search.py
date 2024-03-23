import pytest

from thunder_book.ch05.maze_state import (
    AlternateMazeState,
    MazeParams,
    MCTSParams,
)
from thunder_book.ch05.monte_carlo_tree_search import Node


@pytest.fixture
def state() -> AlternateMazeState:
    params = MazeParams(width=5, height=5, end_turn=10)
    return AlternateMazeState(0, params)


@pytest.fixture
def node(state) -> Node:
    mcts_params = MCTSParams(c=1.0, expand_threshold=10)
    return Node(state, mcts_params)


def test_expand(node):
    node.expand()

    # state of child node should be +1 from parent node
    for child_node in node.child_nodes:
        assert child_node.state.turn == node.state.turn + 1


def test_make_node(node):
    assert node.n == 0
