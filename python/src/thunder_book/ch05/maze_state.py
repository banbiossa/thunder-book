from __future__ import annotations

import copy
import random

import numpy as np
from pydantic import BaseModel

from thunder_book.ch05 import constants


class Character(BaseModel):
    y: int
    x: int
    mark: str
    game_score: int = 0

    def __eq__(self, other: tuple[int, int]) -> bool:
        # util function to compare positions
        return self.y == other[0] and self.x == other[1]


class AlternateMazeState:
    dx: list[int] = [1, -1, 0, 0]
    dy: list[int] = [0, 0, 1, -1]

    def __init__(self, seed: int) -> None:
        random.seed(seed)
        self.turn = 0

        self.characters: list[Character] = [
            Character(
                y=int(constants.H / 2),
                x=int(constants.W / 2) - 1,
                mark="A",
            ),
            Character(
                y=int(constants.H / 2),
                x=int(constants.W / 2) + 1,
                mark="B",
            ),
        ]

        self.points = np.zeros(shape=(constants.H, constants.W), dtype=int)
        for y in range(constants.H):
            for x in range(constants.W):
                if any([c == (y, x) for c in self.characters]):
                    continue
                self.points[y][x] = random.randint(0, 9)

    def is_done(self) -> bool:
        return self.turn == constants.END_TURN

    def legal_actions(self) -> list:
        actions = []
        character = self.characters[0]
        for action in range(4):
            ty = character.y + self.dy[action]
            tx = character.x + self.dx[action]
            if 0 <= ty < constants.H and 0 <= tx < constants.W:
                actions.append(action)
        return actions

    def __str__(self) -> str:
        ss = f"turn:\t{self.turn}\n"
        for character in self.characters:
            ss += f"score({character.mark}):\t{character.game_score}"
            ss += f"\ty: {character.y} x: {character.x}\n"

        for y in range(constants.H):
            ss += "\n"
            for x in range(constants.W):
                is_written = False
                for character in self.characters:
                    if character == (y, x):
                        ss += character.mark
                        is_written = True
                        break
                if not is_written:
                    ss += str(self.points[y][x])

        ss += "\n"
        return ss

    def advance(self, action: int) -> None:
        character = self.characters[0]  # turn % 2 でもよさそう
        character.y += self.dy[action]
        character.x += self.dx[action]

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

    def copy(self) -> AlternateMazeState:
        return copy.deepcopy(self)

    def _winner(self) -> str:
        a = self.characters[0]
        b = self.characters[1]
        if a.game_score == b.game_score:
            return "-"
        if a.game_score > b.game_score:
            return a.mark
        return b.mark

    def white_score(self) -> float:
        match self._winner():
            case "A":
                return 1.0
            case "B":
                return 0.0
            case _:
                return 0.5
