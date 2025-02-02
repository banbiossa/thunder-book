from __future__ import annotations

import copy
import enum
import random
from typing import Callable

import numpy as np
from pydantic import BaseModel


class MazeParams(BaseModel):
    width: int
    height: int
    end_turn: int
    num_characters: int


class Coord(BaseModel):
    x: int = 0
    y: int = 0


class D(enum.Enum):
    dx = [1, -1, 0, 0]
    dy = [0, 0, 1, -1]


class MazeState:
    def __init__(self, seed: int, params: MazeParams) -> None:
        self.params = params
        self.points = np.ndarray((params.height, params.width))
        self.turn = 0
        self.characters: list[Coord] = [Coord() for _ in range(params.num_characters)]
        self.game_score = 0
        self.evaluated_score = 0

        self._init_maze(seed)

    def _init_maze(self, seed: int) -> None:
        random.seed(seed)
        for y in range(self.params.height):
            for x in range(self.params.width):
                self.points[y][x] = random.randint(0, 9)

    def init_characters(self) -> None:
        for character in self.characters:
            character.y = random.randint(0, self.params.height - 1)
            character.x = random.randint(0, self.params.width - 1)

    def transition(self) -> None:
        character = random.choice(self.characters)
        character.y = random.randint(0, self.params.height - 1)
        character.x = random.randint(0, self.params.width - 1)

    def set_character(self, character_id: int, y: int, x: int) -> None:
        self.characters[character_id].y = y
        self.characters[character_id].x = x

    def copy(self) -> MazeState:
        return copy.deepcopy(self)

    def get_score(self, should_print: bool = False) -> int:
        tmp_state = self.copy()
        for character in self.characters:
            tmp_state.points[character.y][character.x] = 0
        while not tmp_state.is_done():
            tmp_state.advance()
            if should_print:
                print(tmp_state)
        return tmp_state.game_score

    def __str__(self) -> str:
        map = f"turn: {self.turn}\n"
        map += f"score: {self.game_score}\n"

        for y in range(self.params.height):
            map += "\n"
            for x in range(self.params.width):
                is_written = False
                for character in self.characters:
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
        return map

    def is_done(self) -> bool:
        return self.turn >= self.params.end_turn

    def move_player(self, character_id: int) -> None:
        character = self.characters[character_id]
        best_point = -np.inf
        best_action = 0
        # greedy
        for action in range(4):
            ty = character.y + D.dy.value[action]
            tx = character.x + D.dx.value[action]
            if 0 <= ty < self.params.height and 0 <= tx < self.params.width:
                if best_point < self.points[ty][tx]:
                    best_point = self.points[ty][tx]
                    best_action = action
        character.y += D.dy.value[best_action]
        character.x += D.dx.value[best_action]

    def advance(self) -> None:
        for i in range(self.params.num_characters):
            self.move_player(i)

        for character in self.characters:
            point = self.points[character.y][character.x]
            self.game_score += point
            self.points[character.y][character.x] = 0

        self.turn += 1


def play_game(name: str, action_func: Callable[[MazeState], MazeState], seed: int):
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    state = MazeState(seed, params)
    end_state = action_func(state)
    score = end_state.get_score(should_print=True)
    print(f"Score of {name} is {score}")
