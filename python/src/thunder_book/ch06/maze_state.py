from __future__ import annotations

import copy
import random

import numpy as np
from pydantic import BaseModel

from thunder_book.ch06 import constants as C


class Character(BaseModel):
    y: int
    x: int
    mark: str
    game_score: int = 0

    def __eq__(self, other: tuple[int, int]) -> bool:
        return self.y == other[0] and self.x == other[1]


class SimulataneousMazeState:
    dx = [1, -1, 0, 0]
    dy = [0, 0, 1, -1]

    def __init__(self, seed: int) -> None:
        self.seed = seed
        random.seed(seed)
        self.turn = 0
        self.characters: tuple[Character, Character] = (
            Character(y=int(C.H / 2), x=int(C.W / 2) - 1, mark="A"),
            Character(y=int(C.H / 2), x=int(C.W / 2) + 1, mark="B"),
        )
        self.points = self._init_maze()

    def _init_maze(self) -> np.ndarray:
        points = np.zeros((C.H, C.W), dtype=int)
        for y in range(C.H):
            for x in range(C.W):
                tx = x
                ty = y
                point = random.randint(0, 9)
                if self.characters[0] == (y, x):
                    continue
                if self.characters[1] == (y, x):
                    continue
                points[ty, tx] = point
                tx = C.W - x - 1
                points[ty, tx] = point

        return points

    def is_done(self) -> bool:
        return C.END_TURN == self.turn

    def _advance(self, player_id: int, action: int) -> None:
        character = self.characters[player_id]
        character.y += self.dy[action]
        character.x += self.dx[action]
        character.game_score += self.points[character.y, character.x]

    def advance(self, action0: int, action1: int) -> None:
        self._advance(0, action0)
        self._advance(1, action1)
        for character in self.characters:
            self.points[character.y, character.x] = 0
        self.turn += 1

    def legal_actions(self, player_id: int) -> list[int]:
        actions = []
        character = self.characters[player_id]
        for action in range(4):
            ty = character.y + self.dy[action]
            tx = character.x + self.dx[action]
            if 0 <= ty < C.H and 0 <= tx < C.W:
                actions.append(action)
        return actions

    def __repr__(self) -> str:
        return str(self)

    def __str__(self) -> str:
        ss = f"turn:\t{self.turn}\n"
        for character in self.characters:
            ss += f"score({character.mark}):\t{character.game_score}"
            ss += f"\ty: {character.y} x: {character.x}\n"

        for y in range(C.H):
            ss += "\n"
            for x in range(C.W):
                ss += self._get_char(y, x)

        ss += "\n"
        return ss

    def _get_char(self, y: int, x: int) -> str:
        # both
        if all([character == (y, x) for character in self.characters]):
            return "X"
        # one
        for character in self.characters:
            if character == (y, x):
                return character.mark
        # point
        return str(self.points[y][x])

    def score(self, player_id: int) -> float:
        score = self.characters[0].game_score - self.characters[1].game_score
        # 先手か後手かでスコアを入れ替える
        if player_id == 1:
            score = 1 - score

        if score == 0:
            return 0.5
        if score > 0:
            return 1.0
        return 0.0

    def copy(self) -> SimulataneousMazeState:
        return copy.deepcopy(self)
