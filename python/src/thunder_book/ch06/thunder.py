from __future__ import annotations

import fire
import numpy as np

from thunder_book.ch06 import constants as C
from thunder_book.ch06.duct import make_duct_f
from thunder_book.ch06.game import many_games
from thunder_book.ch06.maze_state import ActionFunc
from thunder_book.ch06.maze_state import SimulataneousMazeState as State


class TNode:
    def __init__(
        self,
        state: State,
        C=C.C,
        EXPAND_THRESHOLD=C.EXPAND_THRESHOLD,
    ):
        self.state = state.copy()
        self.w = 0
        self.n = 0

        self.child_nodeses = np.array([], dtype=TNode)
        # utility to access child_nodeses
        self.accessor = np.vectorize(lambda node: node.n, otypes=[int])

        self.C = C
        self.EXPAND_THRESHOLD = EXPAND_THRESHOLD

    def next_child_node(self) -> TNode:
        for nodes in self.child_nodeses:
            for node in nodes:
                if node.n == 0:
                    return node

        # get thunder value for each node?
        best_i = self.action0()
        best_j = self.action1()
        return self.child_nodeses[best_i][best_j]

    def action0(self) -> int:
        row_sums = []
        for row in self.child_nodeses:
            w = sum([node.w for node in row])
            n = sum([node.n for node in row])
            row_sums.append(w / n)
        best_index = np.argmax(row_sums)
        return best_index.astype(int)

    def action1(self) -> int:
        cols_sums = []
        for col in self.child_nodeses.T:
            w = 1 - sum([node.w for node in col])
            n = sum([node.n for node in col])
            cols_sums.append(w / n)
        best_index = np.argmax(cols_sums)
        return best_index.astype(int)

    def expand(self) -> None:
        legal_actions0 = self.state.legal_actions(0)
        legal_actions1 = self.state.legal_actions(1)
        nodeses: list[list[TNode]] = []
        for action0 in legal_actions0:
            nodes: list[TNode] = []
            for action1 in legal_actions1:
                nodes.append(TNode(self.state))
                nodes[-1].state.advance(action0, action1)
            nodeses.append(nodes.copy())
        self.child_nodeses = np.array(nodeses.copy())

    def __repr__(self) -> str:
        return f"Node(n={self.n}, w={self.w})"

    def explore(self) -> float:
        if self.state.is_done():
            value = self.state.score(0)
            self._increment(value)
            return value

        if self.child_nodeses.size == 0:
            value = self.state.position_value()
            self._increment(value)
            self.expand()  # always expand if no child nodes
            return value

        # recursive case
        value = self.next_child_node().explore()
        self._increment(value)
        return value

    def _increment(self, value: float):
        self.n += 1
        self.w += value

    def best_i(self) -> int:
        child_nodeses_n = self.accessor(self.child_nodeses)
        return np.argmax(child_nodeses_n.sum(axis=1)).astype(int)

    def best_j(self) -> int:
        child_nodeses_n = self.accessor(self.child_nodeses)
        return np.argmax(child_nodeses_n.sum(axis=0)).astype(int)


def thunder_search_action(state: State, player_id: int, playout_number: int):
    node = TNode(state)
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


def make_thunder_f(playout_number: int) -> ActionFunc:
    def thunder_f(state: State, player_id: int) -> int:
        return thunder_search_action(state, player_id, playout_number)

    return thunder_f


def thunder_vs_duct(num_playout=500, num_games=100):
    print(f"thunder {num_playout} vs duct {num_playout}, {num_games=}")

    thunder_f = make_thunder_f(num_playout)
    duct_f = make_duct_f(num_playout)

    win_rate = many_games(
        num_games,
        (thunder_f, duct_f),
        player_id=0,
        print_every=10,
    )
    print()
    print(f"{win_rate=:.2f} for thunder {num_playout} vs duct {num_playout}")


def main(game="thunder_vs_duct", *args, **kwargs):
    if game == "thunder_vs_duct":
        return thunder_vs_duct(*args, **kwargs)

    raise ValueError(f"Unknown game: {game}")


if __name__ == "__main__":
    fire.Fire(main)
