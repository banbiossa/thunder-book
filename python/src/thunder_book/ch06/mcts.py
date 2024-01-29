from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Generic, TypeVar

import fire
import numpy as np
from thunder_book.ch06 import constants as C
from thunder_book.ch06.game import many_games
from thunder_book.ch06.maze_state import ActionFunc
from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.monte_carlo import make_monte_carlo_f
from thunder_book.ch06.random_action import random_action


def playout(state: State):
    if state.is_done():
        # 結局treeの作り方上、player_id 0でしかスコア計算しない
        return state.score(0)

    state.advance(
        random_action(state, 0),
        random_action(state, 1),
    )
    return playout(state)


# Define a type variable that can be any subclass of BaseNode
T = TypeVar("T", bound="BaseNode")


class BaseNode(ABC, Generic[T]):
    def __init__(self, state: State) -> None:
        self.state = state.copy()
        self.w = 0
        self.n = 0
        self.child_nodes: list[T] = []

    def _increment(self, value: float) -> None:
        self.w += value
        self.n += 1

    def ucb1(self, t: float) -> float:
        return 1 - self.w / self.n + C.C * np.sqrt(2 * np.log(t) / self.n)

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


class EvenNode(BaseNode["OddNode"]):
    def __init__(self, state: State) -> None:
        super().__init__(state)

    def expand(self) -> None:
        # call after player 0 explore finish
        legal_actions = self.state.legal_actions(0)
        self.child_nodes.clear()
        for action in legal_actions:
            self.child_nodes.append(OddNode(self.state, action))

    def explore(self) -> float:
        # evenは末端ノードではないので,必ずexpandする
        if not self.child_nodes:
            self.expand()

        value = self.next_child_node().explore()
        self._increment(value)
        return value


class OddNode(BaseNode["EvenNode"]):
    def __init__(self, state: State, action0: int) -> None:
        super().__init__(state)
        # the action of player 0, just before this node
        self.action0 = action0

    def expand(self) -> None:
        legal_actions_opp = self.state.legal_actions(1)
        self.child_nodes.clear()
        for action1 in legal_actions_opp:
            self.child_nodes.append(EvenNode(self.state))
            assert isinstance(self.action0, int)
            self.child_nodes[-1].state.advance(self.action0, action1)

    def explore(self) -> float:
        # 奇数番目の場合,値を返す
        # 相手のスコアなので　1-value を increment する
        if self.state.is_done():
            return self.state.score(0)

        if self.child_nodes:
            value = self.next_child_node().explore()
            self._increment(1 - value)
            return value

        # no childs, return playout value
        value = playout(self.state)
        if self.n == C.EXPAND_THRESHOLD:
            self.expand()
        self._increment(1 - value)
        return value


class Node:
    def __init__(self, state: State, action0=None) -> None:
        self.state = state.copy()
        self.w = 0
        self.child_nodes: list[Node] = []
        self.n = 0
        # action0 is the action of player 0
        self.action0 = action0

    def _increment(self, value: float) -> None:
        self.w += value
        self.n += 1

    def _explore_odd(self) -> float:
        # 奇数番目の場合,値を返す
        # 相手のスコアなので　1-value を increment する
        if self.state.is_done():
            return self.state.score(0)

        if self.child_nodes:
            value = self.next_child_node().explore()
            self._increment(1 - value)
            return value

        # no childs, return playout value
        # todo: state の increment が必要かも
        value = playout(self.state)
        if self.n == C.EXPAND_THRESHOLD:
            self.expand()
        self._increment(1 - value)
        return value

    def explore(self) -> float:
        # explore_even でもいい
        # 偶数番目では value を increment する
        # evenは末端ノードではないので,必ずexpandする
        if not self.child_nodes:
            self._expand_odd()

        value = self.next_child_node()._explore_odd()
        self._increment(value)
        return value

    def _expand_odd(self) -> None:
        # call after player 1 explore finish
        legal_actions_opp = self.state.legal_actions(1)
        self.child_nodes.clear()
        for action1 in legal_actions_opp:
            self.child_nodes.append(Node(self.state))
            assert isinstance(self.action0, int)
            self.child_nodes[-1].state.advance(self.action0, action1)

    def expand(self) -> None:
        # expand_even でもいい
        # call after player 0 explore finish
        legal_actions = self.state.legal_actions(0)
        self.child_nodes = []
        for action in legal_actions:
            self.child_nodes.append(Node(self.state, action))

    def ucb1(self, t: float) -> float:
        return 1 - self.w / self.n + C.C * np.sqrt(2 * np.log(t) / self.n)

    def next_child_node(self) -> Node:
        for child_node in self.child_nodes:
            if child_node.n == 0:
                return child_node

        # select best based on UCB1
        t = sum(c.n for c in self.child_nodes)
        ucb1s = [c.ucb1(t) for c in self.child_nodes]
        return self.child_nodes[np.argmax(ucb1s)]


def mcts_action(
    state: State,
    playout_number: int,
) -> int:
    node = Node(state)
    node.expand()
    for _ in range(playout_number):
        node.explore()

    legal_actions = state.legal_actions(0)

    # get argmax of n
    num_searched = [c.n for c in node.child_nodes]
    return legal_actions[np.argmax(num_searched)]


def make_mcts_f(playout_number: int) -> ActionFunc:
    def mcts_f(state: State, player_id: int) -> int:
        return mcts_action(state, playout_number)

    return mcts_f


def mcts_vs_monte_carlo(num_playout=1000, num_games=100):
    monte_carlo_f = make_monte_carlo_f(num_playout)
    mcts_f = make_mcts_f(num_playout)

    actions_bw = (mcts_f, monte_carlo_f)
    # actions_bw = (monte_carlo_f, monte_carlo_f)
    win_rate = many_games(num_games, actions_bw, player_id=0, print_every=10)
    print(f"{win_rate=:.2f} for mcts vs monte carlo {num_playout}")


if __name__ == "__main__":
    fire.Fire(mcts_vs_monte_carlo)
