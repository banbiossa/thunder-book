from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Generic, Optional, TypeVar

import fire
import numpy as np
from pydantic import BaseModel

from thunder_book.ch06.game import many_games
from thunder_book.ch06.maze_state import ActionFunc
from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.monte_carlo import make_monte_carlo_f
from thunder_book.ch06.random_action import random_action

"""
def playout(state: State):
    if state.is_done():
        # 結局treeの作り方上、player_id 0でしかスコア計算しない
        return state.score(0)

    state.advance(
        random_action(state, 0),
        random_action(state, 1),
    )
    return playout(state)
"""


class MCTSParams(BaseModel):
    c: float
    expand_threshold: int


class Playout:
    def __init__(self, state: State) -> None:
        """上の playout でstate を渡す際にcopyしなかったバグを
        踏んだので, 絶対に copy するための工夫。
        """
        self.state = state.copy()

    def playout(self) -> float:
        if self.state.is_done():
            return self.state.score(0)

        self.state.advance(
            random_action(self.state, 0),
            random_action(self.state, 1),
        )
        return self.playout()


# Define a type variable that can be any subclass of BaseNode
T = TypeVar("T", bound="BaseNode")


class BaseNode(ABC, Generic[T]):
    def __init__(
        self,
        state: State,
        params: MCTSParams,
        root: Optional[BaseNode] = None,
        is_root: bool = False,
    ) -> None:
        self.params = params
        self.state = state.copy()
        self.w = 0
        self.n = 0
        self.child_nodes: list[T] = []
        self.root = root if root is not None else self
        assert root is not None or is_root, "root node must be set on non-root nodes"

    def _increment(self, value: float) -> None:
        self.w += value
        self.n += 1

    def __repr__(self) -> str:
        return f"{self.__class__.__name__}(w={self.w}, n={self.n})"

    @property
    def find(self) -> None:
        # calls where am i with self
        return print(self.root._where_am_i(self))

    def __str__(self) -> str:
        return self.root._where_am_i(self)

    @property
    def f(self) -> None:
        # helper for debugger
        return self.find

    def _where_am_i(self, target: BaseNode, depth: int = 0) -> str:
        # prints the staus from the root node
        # but adds a <<< to the current node
        stat = "__ " * depth
        mark = " <<<" if self == target else ""
        stat += self.__repr__() + mark + "\n"
        stat += "".join(c._where_am_i(target, depth + 1) for c in self.child_nodes)
        return stat

    def ucb1(self, t: float) -> float:
        # 子のノードの計算をするため、1-w が必要?
        return 1 - self.w / self.n + self.params.c * np.sqrt(2 * np.log(t) / self.n)

    def next_child_node(self) -> T:
        for child_node in self.child_nodes:
            if child_node.n == 0:
                return child_node

        # select best based on UCB1
        t = sum(c.n for c in self.child_nodes)
        ucb1s = [c.ucb1(t) for c in self.child_nodes]
        return self.child_nodes[np.argmax(ucb1s)]

    @abstractmethod
    def explore(self) -> float:
        NotImplemented()

    @abstractmethod
    def expand(self) -> None:
        NotImplemented()


# Node, OddNode, EvenNode で分ける方が
# 抽象化としてはキレイそう
# やったけど player0 じゃないとプレーできなくしてしまったので
# 自分自身とのプレーができない
# それもやりたいなら A_node, B_node とかにして
# 中身は player_id == 0/1 で分岐するようにするとかありそう


class EvenNode(BaseNode["OddNode"]):
    def __init__(
        self,
        state: State,
        params: MCTSParams,
        root: Optional[BaseNode] = None,
        is_root: bool = False,
    ) -> None:
        super().__init__(state, params, root, is_root=is_root)

    def expand(self) -> None:
        # call after player 0 explore finish
        legal_actions = self.state.legal_actions(0)
        self.child_nodes.clear()
        for action in legal_actions:
            self.child_nodes.append(OddNode(self.state, self.params, self.root, action))

    def explore(self) -> float:
        # evenは末端ノードではないので,必ずexpandする
        if not self.child_nodes:
            self.expand()

        value = self.next_child_node().explore()
        self._increment(value)
        return value


class OddNode(BaseNode["EvenNode"]):
    def __init__(self, state: State, params: MCTSParams, root: BaseNode, action0: int) -> None:
        super().__init__(state, params, root)
        # the action of player 0, just before this node
        self.action0 = action0

    def expand(self) -> None:
        legal_actions_opp = self.state.legal_actions(1)
        self.child_nodes.clear()
        for action1 in legal_actions_opp:
            self.child_nodes.append(EvenNode(self.state, self.params, self.root))
            assert isinstance(self.action0, int)
            self.child_nodes[-1].state.advance(self.action0, action1)

    def explore(self) -> float:
        # 奇数番目の場合,値を返す
        # 相手のスコアなので内部で　1-value を increment している
        if self.state.is_done():
            value = self.state.score(0)
            self._increment(1 - value)
            return value

        if self.child_nodes:
            value = self.next_child_node().explore()
            self._increment(1 - value)
            return value

        # no childs, return playout value
        value = Playout(self.state).playout()
        self._increment(1 - value)
        if self.n >= self.params.expand_threshold:
            self.expand()
        return value


def mcts_action(
    state: State,
    player_id: int,
    playout_number: int,
) -> int:
    # safeguard
    if player_id != 0:
        raise RuntimeError("player_id must be 0 for mcts")

    node = EvenNode(state, MCTSParams(c=1.0, expand_threshold=10), is_root=True)
    node.expand()
    for _ in range(playout_number):
        # breakpoint()
        node.explore()

    legal_actions = state.legal_actions(0)

    # get argmax of n
    num_searched = [c.n for c in node.child_nodes]
    return legal_actions[np.argmax(num_searched)]


def make_mcts_f(playout_number: int) -> ActionFunc:
    def mcts_f(state: State, player_id: int) -> int:
        return mcts_action(state, player_id, playout_number)

    return mcts_f


def mcts_vs_monte_carlo(num_playout=100, num_games=100):
    print(f"mcts vs monte carlo {num_playout}")

    monte_carlo_f = make_monte_carlo_f(num_playout)
    mcts_f = make_mcts_f(num_playout)

    actions_bw = (mcts_f, monte_carlo_f)
    win_rate = many_games(num_games, actions_bw, player_id=0, print_every=10)
    print(f"{win_rate=:.2f} for mcts vs monte carlo {num_playout}")


def mcts_vs_random_action(num_playout=100, num_games=100):
    print(f"mcts {num_playout} vs random")
    mcts_f = make_mcts_f(num_playout)
    actions_bw = (mcts_f, random_action)
    win_rate = many_games(num_games, actions_bw, player_id=0, print_every=10)
    print(f"{win_rate=:.2f} for mcts {num_playout} vs random")


def main(game="monte_carlo", *args, **kwargs):
    if game == "random":
        return mcts_vs_random_action(*args, **kwargs)
    if game == "monte_carlo":
        return mcts_vs_monte_carlo(*args, **kwargs)
    raise ValueError(f"{game=} is not supported")


if __name__ == "__main__":
    fire.Fire(main)
