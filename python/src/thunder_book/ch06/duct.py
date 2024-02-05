from __future__ import annotations

import fire
import numpy as np
from thunder_book.ch06 import constants as C
from thunder_book.ch06.game import many_games
from thunder_book.ch06.maze_state import ActionFunc
from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.mcts import make_mcts_f
from thunder_book.ch06.monte_carlo import make_monte_carlo_f, playout


class Node:
    def __init__(
        self,
        state: State,
        C=C.C,
        EXPAND_THRESHOLD=C.EXPAND_THRESHOLD,
    ) -> None:
        self.state = state.copy()
        self.n = 0
        self.w = 0

        self.child_nodeses = np.ndarray((0, 0), dtype=Node)
        self.accessor = np.vectorize(lambda node: node.n, otypes=[int])

        self.C = C
        self.EXPAND_THRESHOLD = EXPAND_THRESHOLD

    def expand(self) -> None:
        # 本当は1行でできそう,
        # state.advance(i, j) for i, j in product(actions0, actions1)
        # 的な感じ. advance が self を返すようにする必要があるけど
        self.child_nodeses = np.ndarray((0, 0), dtype=Node)

        legal_actions0 = self.state.legal_actions(0)
        legal_actions1 = self.state.legal_actions(1)
        nodeses: list[list[Node]] = []
        for action0 in legal_actions0:
            nodes: list[Node] = []
            for action1 in legal_actions1:
                nodes.append(Node(self.state))
                nodes[-1].state.advance(action0, action1)
            nodeses.append(nodes.copy())

        self.child_nodeses = np.array(nodeses.copy())

    def _increment(self, w: float) -> None:
        self.n += 1
        self.w += w

    def explore(self) -> float:
        if self.state.is_done():
            value = self.state.score(0)
            self._increment(value)
            return value
        if self.child_nodeses.size == 0:
            state_copy = self.state.copy()
            value = playout(state_copy, 0)
            self._increment(value)
            if self.n >= self.EXPAND_THRESHOLD:
                self.expand()
            return value
        # base case
        value = self.next_child_node().explore()
        self._increment(value)
        return value

    def __repr__(self) -> str:
        return f"Node(n={self.n}, w={self.w})"

    def next_child_node(self) -> Node:
        # self child where n == 0
        for (i, j), node in np.ndenumerate(self.child_nodeses):
            if node.n == 0:
                return node

        # get best child node based on UCB1
        best_i = self.action0()
        best_j = self.action1()
        return self.child_nodeses[best_i, best_j]

    @property
    def t(self):
        child_nodeses_n = self.accessor(self.child_nodeses)
        return child_nodeses_n.sum()

    def ucb1(self, w, n) -> float:
        return w / n + self.C * np.sqrt(np.log(self.t) / n)

    def action0(self) -> int:
        # get best ucb1 value, but iterate i and sum j
        row_sums = []
        for row in self.child_nodeses:
            w = sum([node.w for node in row])
            n = sum([node.n for node in row])
            ucb1 = self.ucb1(w, n)
            row_sums.append(ucb1)
        best_index = np.argmax(row_sums)
        return best_index.astype(int)

    def action1(self) -> int:
        # get best ucb1 value, but iterate j and sum i
        col_sums = []
        for col in self.child_nodeses.T:
            w = 1 - sum([node.w for node in col])
            n = sum([node.n for node in col])
            ucb1 = self.ucb1(w, n)
            col_sums.append(ucb1)
        best_index = np.argmax(col_sums)
        return best_index.astype(int)

    def best_i(self) -> int:
        child_nodeses_n = self.accessor(self.child_nodeses)
        return np.argmax(child_nodeses_n.sum(axis=1)).astype(int)

    def best_j(self) -> int:
        child_nodeses_n = self.accessor(self.child_nodeses)
        return np.argmax(child_nodeses_n.sum(axis=0)).astype(int)


def duct_action(state: State, player_id: int, playout_number: int) -> int:
    node = Node(state)
    node.expand()
    for _ in range(playout_number):
        node.explore()

    legal_actions = state.legal_actions(player_id)

    if player_id == 0:
        best_i = node.best_i()
        return legal_actions[best_i]
    else:
        best_j = node.best_j()
        return legal_actions[best_j]


def make_duct_f(playout_number: int) -> ActionFunc:
    def duct_f(state: State, player_id: int) -> int:
        return duct_action(state, player_id, playout_number)

    return duct_f


def duct_vs_monte_carlo(num_playout=500, num_games=100):
    print(f"duct {num_playout} vs monte carlo {num_playout}, {num_games=}")

    duct_f = make_duct_f(num_playout)
    monte_carlo_f = make_monte_carlo_f(num_playout)

    win_rate = many_games(
        num_games,
        (duct_f, monte_carlo_f),
        player_id=0,
        print_every=10,
    )
    print()
    print(f"{win_rate=:.2f} for duct {num_playout} vs monte carlo {num_playout}")


def duct_vs_mcts(num_playout=1000, num_games=100):
    print(f"mcts {num_playout} vs duct {num_playout}, {num_games=}")

    # mcts needs to be player 0
    mcts_f = make_mcts_f(num_playout)
    duct_f = make_duct_f(num_playout)

    win_rate = many_games(
        num_games,
        (
            mcts_f,
            duct_f,
        ),
        player_id=0,
        print_every=10,
    )
    print()
    print(f"{win_rate=:.2f} for mcts {num_playout} vs duct {num_playout}")


def main(game="duct_vs_mcts", *args, **kwargs):
    if game == "duct_vs_mcts":
        duct_vs_mcts(*args, **kwargs)
    if game == "duct_vs_monte_carlo":
        duct_vs_monte_carlo(*args, **kwargs)
    raise ValueError(f"unknown game: {game}")


if __name__ == "__main__":
    fire.Fire(main)
