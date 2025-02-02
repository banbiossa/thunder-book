from __future__ import annotations

import logging
from datetime import datetime
from typing import Optional

import fire
import numpy as np

from thunder_book.ch05.average_score import average_score
from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.maze_state import MazeParams, MCTSParams
from thunder_book.ch05.monte_carlo import Playout, primitive_monte_carlo_action
from thunder_book.ch05.time_keeper import TimeKeeper
from thunder_book.util import setup_logging


def mcts_action(
    state: State, playout_number: int, mcts_params: MCTSParams, should_print: bool = False
):
    root_node = Node(state, mcts_params, is_root=True)
    root_node.expand()
    for _ in range(playout_number):
        root_node.evaluate()

    if should_print:
        print(str(root_node))
    return root_node.best_action()


def mcts_action_with_time_threshold(
    state: State,
    time_threshold: int,
    mcst_params: MCTSParams,
    should_print: bool = False,
):
    root_node = Node(state, mcst_params, is_root=True)
    root_node.expand()
    time_keeper = TimeKeeper(time_threshold)
    while not time_keeper.is_time_over():
        root_node.evaluate()
    if should_print:
        print(str(root_node))
    return root_node.best_action()


class Node:
    def __init__(
        self,
        state: State,
        mcts_params: MCTSParams,
        action=None,
        root: Optional[Node] = None,
        is_root: bool = False,
    ) -> None:
        self.state: State = state.copy()
        self.child_nodes: list[Node] = []
        self.n = 0
        self.w = 0
        self.mcts_params = mcts_params

        # to track where the state is
        self.action = action
        self.root = root if root is not None else self
        assert root is not None or is_root, "root node must be set on non-root nodes"

    def expand(self) -> None:
        legal_actions = self.state.legal_actions()
        self.child_nodes = []
        for action in legal_actions:
            self.child_nodes.append(
                Node(
                    self.state,
                    self.mcts_params,
                    action=action,
                    root=self.root,
                )
            )
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
            value = Playout(self.state).playout()
            self._increment(value)
            if self.n == self.mcts_params.expand_threshold:
                self.expand()
            return value

        # base case, has child nodes
        value = 1 - self.next_child_node().evaluate()
        self._increment(value)
        return value

    def ucb1(self, t: float) -> float:
        return 1 - self.w / self.n + self.mcts_params.c * np.sqrt(2 * np.log(t) / self.n)

    def next_child_node(self) -> Node:
        for child_node in self.child_nodes:
            if child_node.n == 0:
                return child_node

        # select best action based on argmax of UCB1
        t = sum(c.n for c in self.child_nodes)
        ucb1 = [c.ucb1(t) for c in self.child_nodes]
        return self.child_nodes[np.argmax(ucb1)]

    def __str__(self) -> str:
        return self.root._where_am_i(self)

    def _where_am_i(self, target: Node, depth: int = 0, action=None) -> str:
        # prints the staus from the root node
        # but adds a <<< to the current node
        ss = ""
        ss += "__ " * depth
        if action is not None:
            ss += f"{action}=>"
        mark = " <<<" if self == target else ""
        ss += f"{self.n}({self.w}){mark}\n"
        for action, child_node in zip(self.state.legal_actions(), self.child_nodes):
            ss += child_node._where_am_i(target, depth + 1, action)
        return ss

    def best_action(self) -> int:
        # select child node with the highest n
        legal_actions = self.state.legal_actions()
        assert len(legal_actions) == len(self.child_nodes)
        best_index = np.argmax([c.n for c in self.child_nodes])

        return legal_actions[best_index]


def mcts_vs_monte_carlo(num_playout: int = 30):
    file_logger = logging.getLogger("file_logger")
    start = datetime.now()
    params = MazeParams(width=5, height=5, end_turn=10)
    mcts_params = MCTSParams(c=1.0, expand_threshold=10)
    mcts_action_f = lambda state: mcts_action(state, num_playout, mcts_params)
    monte_carlo_action_f = lambda state: primitive_monte_carlo_action(state, num_playout)
    num_games = 100
    win_rate = average_score(
        num_games=num_games, actions_wb=(mcts_action_f, monte_carlo_action_f), params=params
    )
    elapsed = (datetime.now() - start).total_seconds()
    print(f"win rate of MCTS vs Monte Carlo in {num_games=}, {num_playout=}: {win_rate}")
    file_logger.info(
        f"| mcts vs. monte carlo {num_playout} | {win_rate*100:.2f}% | {elapsed:.2f}s |"
    )


def mcts_compare(a: int = 100, b: int = 10):
    file_logger = logging.getLogger("file_logger")
    start = datetime.now()
    params = MazeParams(width=5, height=5, end_turn=10)
    mcts_params = MCTSParams(c=1.0, expand_threshold=10)
    mcts_action_a = lambda state: mcts_action(state, a, mcts_params)
    mcts_action_b = lambda state: mcts_action(state, b, mcts_params)
    num_games = 100
    win_rate = average_score(
        num_games=num_games, actions_wb=(mcts_action_a, mcts_action_b), params=params
    )
    elapsed = (datetime.now() - start).total_seconds()
    print(f"win rate of MCTS in {num_games=}, {a=} vs {b=}: {win_rate}")
    file_logger.info(f"| mcts {a} vs. {b} | {win_rate*100:.2f}% | {elapsed:.2f}s |")


def print_tree(num_playout: int = 30):
    params = MazeParams(width=5, height=5, end_turn=10)
    mcts_params = MCTSParams(c=1.0, expand_threshold=10)
    state = State(0, params=params)
    mcts_action(state, num_playout, should_print=True, mcts_params=mcts_params)


def main(game="all", *args, **kwargs):
    ss = f"{game=} "
    ss += " ".join([str(a) for a in args])
    ss += " ".join([f"{k}={v}" for k, v in kwargs.items()])
    print(ss)

    if game == "compare":
        mcts_compare(*args, **kwargs)
        return
    if game == "vs":
        mcts_vs_monte_carlo(*args, **kwargs)
        return
    if game == "print":
        print_tree(*args, **kwargs)
        return

    file_logger = logging.getLogger("file_logger")
    file_logger.info("|name|score|time|")
    file_logger.info("|----|-----|----|")
    mcts_compare(a=100, b=10)
    mcts_vs_monte_carlo(num_playout=3000)


if __name__ == "__main__":
    setup_logging()
    fire.Fire(main)
