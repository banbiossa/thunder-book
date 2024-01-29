from __future__ import annotations

from thunder_book.ch06 import constants as C
from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.random_action import random_action


def playout(state: State, player_id: int):
    if state.is_done():
        return state.score(player_id)

    state.advance(
        random_action(state, 0),
        random_action(state, 1),
    )
    return playout(state, player_id)


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

    def _explore_odd(self, player_id) -> float:
        # 奇数番目の場合,値を返す
        # value の　increment しない
        if self.state.is_done_alternate():
            return self.state.score(player_id)

        if self.child_nodes:
            value = self.next_child_node().explore()
            self.n += 1
            return value

        # no childs, return playout value
        # todo: state の increment が必要かも
        value = playout(self.state, player_id)
        if self.n == C.EXPAND_THRESHOLD:
            self.expand_even()
        self.n += 1
        return value

    def explore(self, player_id) -> float:
        # explore_even でもいい
        # 偶数番目では value を increment する
        # evenは末端ノードではないので,必ずexpandする
        if not self.child_nodes:
            self.expand_odd()

        value = self.next_child_node()._explore_odd()
        # increment
        self.w += value
        self.n += 1
        return value

    def expand_odd(self) -> None:
        # call after player 1 explore finish
        legal_actions_opp = self.state.legal_actions(1)
        self.child_nodes = []
        for action1 in legal_actions_opp:
            self.child_nodes.append(Node(self.state))
            assert isinstance(self.action0, int)
            self.child_nodes[-1].state.advance(self.action0, action1)

    def expand_even(self) -> None:
        # call after player 0 explore finish
        legal_actions = self.state.legal_actions(0)
        self.child_nodes = []
        for action in legal_actions:
            self.child_nodes.append(Node(self.state, action))
