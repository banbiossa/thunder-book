from __future__ import annotations

import numpy as np
import fire

from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.monte_carlo import playout, primitive_monte_carlo_action
from thunder_book.ch05 import constants
from thunder_book.ch05.average_score import average_score


def mcts_action(state: State, playout_number: int):
    root_node = Node(state)
    root_node.expand()
    for _ in range(playout_number):
        root_node.evaluate()

    # select child node with the highest n
    legal_actions = state.legal_actions()
    assert len(legal_actions) == len(root_node.child_nodes)
    best_index = np.argmax([c.n for c in root_node.child_nodes])
    return legal_actions[best_index]


class Node:
    def __init__(self, state: State) -> None:
        self.state: State = state.copy()
        self.child_nodes: list[Node] = []
        self.n = 0
        self.w = 0

    def expand(self) -> None:
        legal_actions = self.state.legal_actions()
        self.child_nodes = []
        for action in legal_actions:
            self.child_nodes.append(Node(self.state))
            self.child_nodes[-1].state.advance(action)

    def _increment(self, value: float) -> float:
        self.w += value
        self.n += 1
        return value

    def evaluate(self) -> float:
        if self.state.is_done():
            value = self.state.teban_score()
            return self._increment(value)

        # no child nodes
        if not self.child_nodes:
            state_copy = self.state.copy()
            value = playout(state_copy)
            if self.n == constants.EXPAND_THRESHOLD:
                self.expand()
            return self._increment(value)

        # base case, has child nodes
        value = 1 - self.next_child_node().evaluate()
        return self._increment(value)

    def ucb1(self, t: float) -> float:
        return 1 - self.w / self.n + constants.C * np.sqrt(2 * np.log(t) / self.n)

    def next_child_node(self) -> Node:
        for child_node in self.child_nodes:
            if child_node.n == 0:
                return child_node

        # select best action based on argmax of UCB1
        t = sum(c.n for c in self.child_nodes)
        ucb1 = [c.ucb1(t) for c in self.child_nodes]
        return self.child_nodes[np.argmax(ucb1)]


def mcts_vs_monte_carlo(num_playout: int = 30):
    mcts_action_f = lambda state: mcts_action(state, num_playout)
    monte_carlo_action_f = lambda state: primitive_monte_carlo_action(
        state, num_playout
    )
    num_games = 100
    win_rate = average_score(
        num_games=num_games, actions_wb=(mcts_action_f, monte_carlo_action_f)
    )
    print(
        f"win rate of MCTS vs Monte Carlo in {num_games=}, {num_playout=}: {win_rate}"
    )


if __name__ == "__main__":
    fire.Fire(mcts_vs_monte_carlo)
