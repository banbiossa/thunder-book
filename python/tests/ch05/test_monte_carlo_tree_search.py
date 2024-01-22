from thunder_book.ch05.monte_carlo_tree_search import Node
from thunder_book.ch05.maze_state import AlternateMazeState as State


def test_expand():
    state = State(0)
    node = Node(state)
    node.expand()

    # state of child node should be +1 from parent node
    for child_node in node.child_nodes:
        assert child_node.state.turn == node.state.turn + 1
