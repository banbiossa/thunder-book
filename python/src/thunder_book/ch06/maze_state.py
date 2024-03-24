from __future__ import annotations

import copy
import enum
import random
from typing import Callable

import numpy as np
from pydantic import BaseModel


class Character(BaseModel):
    y: int
    x: int
    mark: str
    game_score: int = 0

    def on(self, y, x) -> bool:
        return self.y == y and self.x == x


class D(enum.Enum):
    dx = [1, -1, 0, 0]
    dy = [0, 0, 1, -1]


class MazeParams(BaseModel):
    height: int
    width: int
    end_turn: int


class SimulataneousMazeState:
    def __init__(self, seed: int, params: MazeParams) -> None:
        self.params = params
        self.seed = seed
        random.seed(seed)
        self.turn = 0
        self.characters: tuple[Character, Character] = (
            Character(y=int(self.params.height / 2), x=int(self.params.width / 2) - 1, mark="A"),
            Character(y=int(self.params.height / 2), x=int(self.params.width / 2) + 1, mark="B"),
        )
        self.points = self._init_maze()

    def _init_maze(self) -> np.ndarray:
        points = np.zeros((self.params.height, self.params.width), dtype=int)
        for y in range(self.params.height):
            for x in range(self.params.width):
                tx = x
                ty = y
                point = random.randint(0, 9)
                if any([c.on(y, x) for c in self.characters]):
                    continue
                points[ty, tx] = point
                tx = self.params.width - x - 1
                points[ty, tx] = point

        return points

    def is_done(self) -> bool:
        return self.turn >= self.params.end_turn

    def position_value(self) -> float:
        # いわゆる評価値. [0, 1]で返す
        denominator = sum([c.game_score for c in self.characters])
        nominator = self.characters[0].game_score
        if denominator == 0:
            return 0.5
        return nominator / denominator

    def _advance(self, player_id: int, action: int) -> None:
        character = self.characters[player_id]
        character.y += D.dy.value[action]
        character.x += D.dx.value[action]
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
            ty = character.y + D.dy.value[action]
            tx = character.x + D.dy.value[action]
            if 0 <= ty < self.params.height and 0 <= tx < self.params.width:
                actions.append(action)
        return actions

    def __str__(self) -> str:
        ss = f"turn: {self.turn}\n"
        for character in self.characters:
            ss += f"score({character.mark}): {character.game_score}"
            ss += f" y: {character.y} x: {character.x}\n"

        for y in range(self.params.height):
            ss += "\n"
            for x in range(self.params.width):
                ss += self._get_char(y, x)

        ss += "\n"
        return ss

    def _get_char(self, y: int, x: int) -> str:
        # both
        if all([character.on(y, x) for character in self.characters]):
            return "X"
        # one
        for character in self.characters:
            if character.on(y, x):
                return character.mark
        # point
        if self.points[y][x] == 0:
            return "."
        return str(self.points[y][x])

    def score(self, player_id: int) -> float:
        diff = self.characters[0].game_score - self.characters[1].game_score
        # 先手か後手かでスコアを入れ替える
        if player_id == 1:
            diff = -diff

        if diff == 0:
            return 0.5
        if diff > 0:
            return 1.0
        return 0.0

    def copy(self) -> SimulataneousMazeState:
        return copy.deepcopy(self)


# State & player_id -> action
ActionFunc = Callable[[SimulataneousMazeState, int], int]
