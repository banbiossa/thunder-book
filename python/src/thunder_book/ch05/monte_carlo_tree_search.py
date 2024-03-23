from __future__ import annotations

import fire
import numpy as np

from thunder_book.ch05.average_score import average_score
from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.maze_state import MazeParams, MCTSParams
from thunder_book.ch05.monte_carlo import playout, primitive_monte_carlo_action
from thunder_book.ch05.time_keeper import TimeKeeper


def mcts_action(
    state: State, playout_number: int, mcts_params: MCTSParams, should_print: bool = False
):
    root_node = Node(state, mcts_params)
    root_node.expand()
    for _ in range(playout_number):
        root_node.evaluate()

    if should_print:
        print(root_node.print_tree())
    return root_node.best_action()


def mcts_action_with_time_threshold(state: State, time_threshold: int, mcst_params: MCTSParams):
    root_node = Node(state, mcst_params)
    root_node.expand()
    time_keeper = TimeKeeper(time_threshold)
    while not time_keeper.is_time_over():
        root_node.evaluate()
    return root_node.best_action()


class Node:
    def __init__(self, state: State, mcts_params: MCTSParams) -> None:
        self.state: State = state.copy()
        self.child_nodes: list[Node] = []
        self.n = 0
        self.w = 0
        self.mcts_params = mcts_params

    def expand(self) -> None:
        legal_actions = self.state.legal_actions()
        self.child_nodes = []
        for action in legal_actions:
            self.child_nodes.append(Node(self.state, self.mcts_params))
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
            if self.n == self.mcts_params.expand_threshold:
                self.expand()
            return self._increment(value)

        # base case, has child nodes
        value = 1 - self.next_child_node().evaluate()
        return self._increment(value)

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

    def print_tree(self, depth: int = 0) -> str:
        ss = ""
        for i, child_node in enumerate(self.child_nodes):
            ss += "__" * depth
            ss += f" {i}({child_node.n})\n"
            if child_node.child_nodes:
                ss += child_node.print_tree(depth + 1)
        return ss

    def __str__(self) -> str:
        return self.print_tree()

    def best_action(self) -> int:
        # select child node with the highest n
        legal_actions = self.state.legal_actions()
        assert len(legal_actions) == len(self.child_nodes)
        best_index = np.argmax([c.n for c in self.child_nodes])

        return legal_actions[best_index]


def mcts_vs_monte_carlo(num_playout: int = 30):
    params = MazeParams(width=5, height=5, end_turn=10)
    mcts_params = MCTSParams(c=1.0, expand_threshold=10)
    mcts_action_f = lambda state: mcts_action(state, num_playout, mcts_params)
    monte_carlo_action_f = lambda state: primitive_monte_carlo_action(state, num_playout)
    num_games = 100
    win_rate = average_score(
        num_games=num_games, actions_wb=(mcts_action_f, monte_carlo_action_f), params=params
    )
    print(f"win rate of MCTS vs Monte Carlo in {num_games=}, {num_playout=}: {win_rate}")


def mcts_compare(a: int = 100, b: int = 10):
    params = MazeParams(width=5, height=5, end_turn=10)
    mcts_params = MCTSParams(c=1.0, expand_threshold=10)
    mcts_action_a = lambda state: mcts_action(state, a, mcts_params)
    mcts_action_b = lambda state: mcts_action(state, b, mcts_params)
    num_games = 100
    win_rate = average_score(
        num_games=num_games, actions_wb=(mcts_action_a, mcts_action_b), params=params
    )
    print(f"win rate of MCTS in {num_games=}, {a=} vs {b=}: {win_rate}")


def print_tree(num_playout: int = 30):
    params = MazeParams(width=5, height=5, end_turn=10)
    mcts_params = MCTSParams(c=1.0, expand_threshold=10)
    state = State(0, params=params)
    mcts_action(state, num_playout, should_print=True, mcts_params=mcts_params)


def main(game="compare", *args, **kwargs):
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
    raise KeyError(f"unknown game: {game}")


if __name__ == "__main__":
    fire.Fire(main)
