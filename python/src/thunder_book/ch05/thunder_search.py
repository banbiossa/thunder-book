from __future__ import annotations

import numpy as np
import fire

from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.monte_carlo_tree_search import (
    mcts_action,
    mcts_action_with_time_threshold,
)
from thunder_book.ch05.average_score import average_score
from thunder_book.ch05.time_keeper import TimeKeeper
from thunder_book.ch05.iterative_deepening import iterative_deepening_action


class TNode:
    def __init__(self, state: State) -> None:
        """
        node for thunder search
        use "TNode" to avoid name conflict with "Node" in monte_carlo_tree_search.py
        """
        self.state: State = state.copy()
        self.child_nodes: list[TNode] = []
        self.n = 0
        self.w = 0

    def expand(self) -> None:
        legal_actions = self.state.legal_actions()
        self.child_nodes = []
        for action in legal_actions:
            self.child_nodes.append(TNode(self.state))
            self.child_nodes[-1].state.advance(action)

    def _increment(self, value: float) -> None:
        self.w += value
        self.n += 1

    def evaluate(self) -> float:
        if self.state.is_done():
            value = self.state.teban_score()
            self._increment(value)
            return value

        # no child nodes
        if not self.child_nodes:
            value = self.state.get_score_rate()
            self._increment(value)
            self.expand()
            return value

        # base case, has child nodes
        value = 1 - self.next_child_node().evaluate()
        self._increment(value)
        return value

    def next_child_node(self) -> TNode:
        # select child node that has not been evaluated
        for child_node in self.child_nodes:
            if child_node.n == 0:
                return child_node

        # select child node with the highest thunder_value
        thunder_values = [(1 - c.w / c.n) for c in self.child_nodes]
        best_index = np.argmax(thunder_values)
        return self.child_nodes[best_index]

    def best_action(self) -> int:
        legal_actions = self.state.legal_actions()
        assert len(legal_actions) == len(self.child_nodes)
        # select node with the highest n
        best_index = np.argmax([c.n for c in self.child_nodes])
        return legal_actions[best_index]


def thunder_search_action(state: State, playout_number: int):
    root_node = TNode(state)
    root_node.expand()
    for _ in range(playout_number):
        root_node.evaluate()
    return root_node.best_action()


def thunder_search_action_with_time_threshold(state: State, time_threshold: int):
    root_node = TNode(state)
    root_node.expand()
    time_keeper = TimeKeeper(time_threshold)
    while not time_keeper.is_time_over():
        root_node.evaluate()
    return root_node.best_action()


def thunder_search_vs_mcts(num_playouts=100):
    num_games = 100
    thunder_search_f = lambda x: thunder_search_action(x, num_playouts)
    mcts_action_f = lambda x: mcts_action(x, num_playouts)

    actions_wb = (thunder_search_f, mcts_action_f)
    win_rate = average_score(num_games, actions_wb)

    print(f"thunder_search vs mcts: {win_rate:.2f} in {num_playouts=}, {num_games=}")


def thunder_vs_mcts_timebound(time_threshold=1):
    num_games = 100
    thunder_search_f = lambda x: thunder_search_action_with_time_threshold(
        x, time_threshold
    )
    mcts_action_f = lambda x: mcts_action_with_time_threshold(x, time_threshold)
    actions_wb = (thunder_search_f, mcts_action_f)
    win_rate = average_score(num_games, actions_wb)

    print(
        f"thunder_search vs mcts timebound: {win_rate:.2f} in {time_threshold=}, {num_games=}"
    )


def thunder_vs_iterative_deepening_timebound(time_threshold=1):
    num_games = 100
    thunder_search_f = lambda x: thunder_search_action_with_time_threshold(
        x, time_threshold
    )
    iterative_deepening_action_f = lambda x: iterative_deepening_action(
        x, time_threshold
    )
    actions_wb = (thunder_search_f, iterative_deepening_action_f)
    win_rate = average_score(num_games, actions_wb)

    print(
        f"thunder_search vs iterative deepening timebound: {win_rate:.2f} in {time_threshold=}, {num_games=}"
    )


def main(game="time", *args, **kwargs):
    if game == "time":
        return thunder_vs_mcts_timebound(*args, **kwargs)
    if game == "compare":
        return thunder_search_vs_mcts(*args, **kwargs)
    if game == "alpha_beta":
        return thunder_vs_iterative_deepening_timebound(*args, **kwargs)


if __name__ == "__main__":
    fire.Fire(main)
