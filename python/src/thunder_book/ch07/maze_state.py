from __future__ import annotations

import copy
from typing import Annotated, Callable, Literal

import numpy as np
from numpy.typing import NDArray
from pydantic import BaseModel
from thunder_book.ch07 import constants as C


class Character(BaseModel):
    y: int
    x: int


class DistanceCoord(BaseModel):
    y: int
    x: int
    distance: int

    @staticmethod
    def from_character(character: Character) -> DistanceCoord:
        return DistanceCoord(
            y=character.y,
            x=character.x,
            distance=0,
        )


MazeShape = Annotated[NDArray[np.int16], Literal["C.H", "C.W"]]


class WallMazeState:
    dx = [1, -1, 0, 0]
    dy = [0, 0, 1, -1]

    def __init__(self, seed: int):
        np.random.seed(seed)
        self.turn = 0
        self.game_score = 0
        self.evaluated_score = 0
        self.character = Character(y=0, x=0)
        self.walls: MazeShape = self._init_maze()
        self.points: MazeShape = self._init_points()
        self.first_action: int = -1

    def _init_maze(self) -> MazeShape:
        walls = np.zeros((C.H, C.W), dtype=int)
        for y in range(1, C.H, 2):
            for x in range(1, C.W, 2):
                if self.character.y == y and self.character.x == x:
                    continue
                walls[y, x] = 1

                direction_size = 4 if y == 1 else 3
                direction = np.random.randint(0, direction_size)
                ty = y + self.dy[direction]
                tx = x + self.dx[direction]
                if self.character.y == ty and self.character.x == tx:
                    continue
                walls[ty, tx] = 1
        return walls

    def _init_points(self) -> MazeShape:
        points = np.zeros((C.H, C.W), dtype=int)
        for y in range(C.H):
            for x in range(C.W):
                if self.character.y == y and self.character.x == x:
                    continue
                if self.walls[y, x] == 1:
                    continue
                points[y, x] = np.random.randint(0, 10)
        return points

    def legal_actions(self) -> list[int]:
        actions: list[int] = []
        for action in range(4):
            ty = self.character.y + self.dy[action]
            tx = self.character.x + self.dx[action]
            if ty >= 0 and ty < C.H and tx >= 0 and tx < C.W and self.walls[ty, tx] == 0:
                actions.append(action)
        return actions

    def is_legal(self) -> bool:
        return self.walls[self.character.y, self.character.x] == 0

    def __str__(self) -> str:
        ss = f"turn:\t{self.turn}\n"
        ss += f"score:\t{self.game_score}\n"
        for h in range(C.H):
            ss += "\n"
            for w in range(C.W):
                if self.character.y == h and self.character.x == w:
                    ss += "@"
                elif self.walls[h, w] == 1:
                    ss += "#"
                elif self.points[h, w] > 0:
                    ss += self.points[h, w].astype(str)
                else:
                    ss += "."
        ss += "\n"
        return ss

    def __repr__(self) -> str:
        return str(self)

    def is_done(self) -> bool:
        return self.turn >= C.END_TURN

    def advance(self, action: int) -> None:
        self.character.y += self.dy[action]
        self.character.x += self.dx[action]
        self.game_score += self.points[self.character.y, self.character.x]
        self.points[self.character.y, self.character.x] = 0
        self.turn += 1

    def evaluate_score(self) -> None:
        self.evaluated_score = self.game_score

    def __lt__(self, other: WallMazeState) -> bool:
        return self.evaluated_score < other.evaluated_score

    def copy(self) -> WallMazeState:
        return copy.deepcopy(self)

    def get_distance_to_nearest_point(self) -> int:
        que = []
        que.append(DistanceCoord.from_character(self.character))
        checked = np.zeros((C.H, C.W), dtype=bool)
        while que:
            coord = que.pop(0)
            if self.points[coord.y, coord.x] > 0:
                return coord.distance
            checked[coord.y, coord.x] = True

            for action in range(4):
                ty = coord.y + self.dy[action]
                tx = coord.x + self.dx[action]
                if (
                    ty >= 0
                    and ty < C.H
                    and tx >= 0
                    and tx < C.W
                    and self.walls[ty, tx] == 0
                    and not checked[ty, tx]
                ):
                    que.append(
                        DistanceCoord(
                            y=ty,
                            x=tx,
                            distance=coord.distance + 1,
                        )
                    )
        # max is all maze
        return C.H * C.W


ActionFunc = Callable[[WallMazeState], int]
