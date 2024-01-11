from __future__ import annotations

import random
import copy

import numpy as np
from pydantic import BaseModel

from thunder_book.ch04 import constants


class Coord(BaseModel):
    x: int = 0
    y: int = 0


class MazeState:
    dx = [1, -1, 0, 0]
    dy = [0, 0, 1, -1]

    def __init__(self, seed: int) -> None:
        self.points = np.ndarray((constants.H, constants.W))
        self.turn = 0
        self.chracters: list[Coord] = [Coord() for _ in range(constants.CHARACTER_N)]
        self.game_score = 0
        self.evaluated_score = 0

        self._init_maze(seed)

    def _init_maze(self, seed: int) -> None:
        random.seed(seed)
        for y in range(constants.H):
            for x in range(constants.W):
                self.points[y][x] = random.randint(0, 9)

    def set_character(self, character_id: int, y: int, x: int) -> None:
        self.chracters[character_id].y = y
        self.chracters[character_id].x = x

    def copy(self) -> MazeState:
        return copy.deepcopy(self)

    def get_score(self, should_print: bool = False) -> int:
        tmp_state = self.copy()
        for character in self.chracters:
            tmp_state.points[character.y][character.x] = 0
        while not tmp_state.is_done():
            tmp_state.advance()
            if should_print:
                print(tmp_state)
        return tmp_state.game_score

    def __str__(self) -> str:
        map = "\n"
        map += f"turn:\t{self.turn}\n"
        map += f"score:\t{self.game_score}\n"
        map += "=" * (constants.W + 2) + "\n"
        for y in range(constants.H):
            for x in range(constants.W):
                is_written = False
                for character in self.chracters:
                    if y == character.y and x == character.x:
                        map += "@"
                        is_written = True
                        break
                if not is_written:
                    if self.points[y][x] > 0:
                        map += f"{int(self.points[y][x])}"
                    else:
                        map += "."
            map += "\n"
        map += "=" * (constants.W + 2) + "\n"
        return map

    def is_done(self) -> bool:
        return self.turn >= constants.END_TURN

    def move_player(self, character_id: int) -> None:
        character = self.chracters[character_id]
        best_point = -np.inf
        best_action = 0
        # greedy
        for action in range(4):
            ty = character.y + self.dy[action]
            tx = character.x + self.dx[action]
            if 0 <= ty < constants.H and 0 <= tx < constants.W:
                if best_point < self.points[ty][tx]:
                    best_point = self.points[ty][tx]
                    best_action = action
        character.y += self.dy[best_action]
        character.x += self.dx[best_action]

    def advance(self) -> None:
        for i in range(constants.CHARACTER_N):
            self.move_player(i)

        for character in self.chracters:
            point = self.points[character.y][character.x]
            self.game_score += point
            self.points[character.y][character.x] = 0

        self.turn += 1
