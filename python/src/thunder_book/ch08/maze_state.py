from __future__ import annotations

import abc
import copy
import enum
from typing import Callable

import numpy as np
from pydantic import BaseModel


class Stone(BaseModel):
    x: int
    y: int


class Status(enum.Enum):
    ONGOING = "ongoing"
    WIN = "win"
    LOSE = "lose"
    DRAW = "draw"


class D(enum.Enum):
    UP = [1, -1]
    DOWN = [-1, 1]
    STAY = [0, 0]


class MazeParams(BaseModel):
    width: int
    height: int


type ActionFunc = Callable[[ConnectFourState], int]


class MazeState(abc.ABC):
    @abc.abstractmethod
    def copy(self) -> MazeState:
        pass

    @abc.abstractmethod
    def is_done(self) -> bool:
        pass

    @abc.abstractmethod
    def get_status(self) -> Status:
        pass

    @abc.abstractmethod
    def legal_actions(self) -> list[int]:
        pass

    @abc.abstractmethod
    def advance(self, action: int) -> None:
        pass

    @abc.abstractmethod
    def teban_score(self) -> float:
        pass

    @abc.abstractmethod
    def white_score(self) -> float:
        pass


class ConnectFourState(MazeState):
    def __init__(self, params: MazeParams) -> None:
        self.is_first = True
        self.my_board = np.zeros((params.height, params.width), dtype=bool)
        self.enemy_board = np.zeros((params.height, params.width), dtype=bool)
        self.status: Status = Status.ONGOING
        self.params = params

    def copy(self) -> ConnectFourState:
        return copy.deepcopy(self)

    def is_done(self) -> bool:
        return self.status != Status.ONGOING

    def get_status(self) -> Status:
        return self.status

    def legal_actions(self) -> list[int]:
        actions = []
        for x in range(self.params.width):
            for y in range(self.params.height - 1, -1, -1):
                if not self.my_board[y][x] and not self.enemy_board[y][x]:
                    actions.append(x)
                    break
        return actions

    def advance(self, action: int) -> None:
        stone = self.place_stone(action)

        # 横方向
        self.check_connection(stone, D.UP, D.STAY)

        # 斜め上方向
        if not self.is_done():
            self.check_connection(stone, D.UP, D.UP)
        # 斜め下方向
        if not self.is_done():
            self.check_connection(stone, D.UP, D.DOWN)
        # 下方向
        if not self.is_done():
            self.check_connection(stone, D.STAY, D.UP)

        # swap my_board and enemy_board
        self.my_board, self.enemy_board = self.enemy_board, self.my_board
        self.is_first = not self.is_first

        # 終了判定
        if not self.is_done() and len(self.legal_actions()) == 0:
            self.status = Status.DRAW

    def place_stone(self, action: int) -> Stone:
        for y in range(self.params.height):
            if not self.my_board[y][action] and not self.enemy_board[y][action]:
                self.my_board[y][action] = True
                # break to keep type checker happy
                break

        return Stone(x=action, y=y)

    def check_connection(self, first_stone: Stone, dx: D, dy: D) -> bool:
        que = [first_stone]
        check = np.zeros((self.params.height, self.params.width), dtype=bool)
        count = 0

        while que:
            stone = que.pop(0)
            count += 1
            if count >= 4:
                # 相手視点は負け
                self.status = Status.LOSE
                return True
            check[stone.y][stone.x] = True

            for action in range(2):
                ty = stone.y + dy.value[action]
                tx = stone.x + dx.value[action]
                if (
                    ty >= 0
                    and ty < self.params.height
                    and tx >= 0
                    and tx < self.params.width
                    and self.my_board[ty][tx]
                    and not check[ty][tx]
                ):
                    que.append(Stone(x=tx, y=ty))
        return False

    def to_string(self) -> str:
        ss = f"is_first: {self.is_first}\n"
        for y in range(self.params.height - 1, -1, -1):
            ss += "\n"
            for x in range(self.params.width):
                if self.my_board[y][x]:
                    ss += "X" if self.is_first else "O"
                elif self.enemy_board[y][x]:
                    ss += "O" if self.is_first else "X"
                else:
                    ss += "."

        ss += "\n"
        return ss

    def teban_score(self) -> float:
        match self.status:
            case Status.WIN:
                return 1.0
            case Status.LOSE:
                return 0.0
            case Status.DRAW:
                return 0.5
            case _:
                # should not reach here
                return 0.5

    def white_score(self) -> float:
        score = self.teban_score()
        if not self.is_first:
            return 1 - score
        return score
