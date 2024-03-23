from __future__ import annotations

import copy
import enum
import random

import numpy as np
from pydantic import BaseModel


class MazeParams(BaseModel):
    width: int
    height: int
    end_turn: int


class MCTSParams(BaseModel):
    c: float
    expand_threshold: int


class Character(BaseModel):
    y: int
    x: int
    mark: str
    game_score: int = 0

    def on(self, y: int, x: int) -> bool:
        # util function to compare positions
        return self.y == y and self.x == x


class D(enum.Enum):
    dx: list[int] = [1, -1, 0, 0]
    dy: list[int] = [0, 0, 1, -1]


class AlternateMazeState:
    def __init__(self, seed: int, params: MazeParams) -> None:
        self.params = params
        random.seed(seed)
        self.turn = 0

        self.characters: list[Character] = [
            Character(
                y=int(self.params.height / 2),
                x=int(self.params.width / 2) - 1,
                mark="A",
            ),
            Character(
                y=int(self.params.height / 2),
                x=int(self.params.width / 2) + 1,
                mark="B",
            ),
        ]

        self.points = np.zeros(shape=(self.params.height, self.params.width), dtype=int)
        for y in range(self.params.height):
            for x in range(self.params.width):
                if any([c.on(y, x) for c in self.characters]):
                    continue
                self.points[y][x] = random.randint(0, 9)

    def is_done(self) -> bool:
        return self.turn >= self.params.end_turn

    def legal_actions(self) -> list:
        actions = []
        character = self.characters[0]
        for action in range(4):
            ty = character.y + D.dy.value[action]
            tx = character.x + D.dx.value[action]
            if 0 <= ty < self.params.height and 0 <= tx < self.params.width:
                actions.append(action)
        return actions

    def __repr__(self) -> str:
        return str(self)

    def __str__(self) -> str:
        ss = f"turn:\t{self.turn}\n"
        for character in self.characters:
            ss += f"score({character.mark}):\t{character.game_score}"
            ss += f"\ty: {character.y} x: {character.x}\n"

        for y in range(self.params.height):
            ss += "\n"
            for x in range(self.params.width):
                is_written = False
                for character in self.characters:
                    if character.on(y, x):
                        ss += character.mark
                        is_written = True
                        break
                if not is_written:
                    ss += str(self.points[y][x])

        ss += "\n"
        return ss

    def advance(self, action: int) -> None:
        character = self.characters[0]  # turn % 2 でもよさそう
        character.y += D.dy.value[action]
        character.x += D.dx.value[action]

        point = self.points[character.y][character.x]
        if point > 0:
            character.game_score += point
            self.points[character.y][character.x] = 0
        self.turn += 1

        # swap characters
        self.characters = self.characters[::-1]

    def print_end_game(self) -> None:
        if not self.is_done():
            print("GAME IS NOT OVER")
            return
        if self.characters[0].game_score == self.characters[1].game_score:
            print("DRAW")
            return
        if self.characters[0].game_score > self.characters[1].game_score:
            print(f"WIN {self.characters[0].mark}")
            return
        else:
            print(f"WIN {self.characters[1].mark}")
            return

    def get_score(self) -> int:
        return self.characters[0].game_score - self.characters[1].game_score

    def teban_score(self) -> float:
        if self.get_score() == 0:
            return 0.5
        if self.get_score() > 0:
            return 1.0
        return 0.0

    def copy(self) -> AlternateMazeState:
        return copy.deepcopy(self)

    def _winner(self) -> str:
        a = self.characters[0]
        b = self.characters[1]
        if a.game_score > b.game_score:
            return a.mark
        if a.game_score < b.game_score:
            return b.mark
        return "-"

    def white_score(self) -> float:
        match self._winner():
            case "A":
                return 1.0
            case "B":
                return 0.0
            case _:
                return 0.5

    def get_score_rate(self) -> float:
        denominator = sum([c.game_score for c in self.characters])
        nominator = self.characters[0].game_score
        if denominator == 0:
            return 0.5
        return nominator / denominator
