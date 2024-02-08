from __future__ import annotations

from typing import Annotated, Literal

import numpy as np
from numpy.typing import NDArray
from pydantic import BaseModel
from thunder_book.ch07 import constants as C


class Character(BaseModel):
    y: int
    x: int


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

    def _init_maze(self) -> MazeShape:
        walls = np.zeros((C.H, C.W), dtype=int)
        for y in range(1, C.H, 2):
            for x in range(1, C.W, 2):
                if self.character.y == y and self.character.x == x:
                    continue
                walls[y, x] = 1

                direction_size = 4 if y == 1 else 3
                direction = np.random.randint(0, direction_size)
                tx = x + self.dx[direction]
                ty = y + self.dy[direction]
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
                points[y, x] = np.random.randint(0, 10)
        return points

    def legal_actions(self) -> list[int]:
        actions: list[int] = []
        for action in range(4):
            ty = self.character.y + self.dy[action]
            tx = self.character.x + self.dx[action]
            if (
                ty >= 0
                and ty < C.H
                and tx >= 0
                and tx < C.W
                and self.walls[ty, tx] == 0
            ):
                actions.append(action)
        return actions

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

    def random_action(self) -> int:
        legal_actions = self.legal_actions()
        return np.random.choice(legal_actions)


def play_game(seed: int) -> None:
    state = WallMazeState(seed)
    print(state)
    while not state.is_done():
        action = state.random_action()
        state.advance(action)
        print(state)


if __name__ == "__main__":
    play_game(0)
