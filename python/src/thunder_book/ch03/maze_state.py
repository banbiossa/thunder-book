import random
from typing import NamedTuple

import numpy as np
from pydantic import BaseModel


class Coord(BaseModel):
    x: int = 0
    y: int = 0


class MazeState:
    dx = [1, -1, 0, 0]
    dy = [0, 0, 1, -1]

    def __init__(self, seed: int, H=3, W=4, END_TURN=4):
        self.H, self.W = H, W
        self.END_TURN = END_TURN
        self.points = np.ndarray((self.H, self.W))
        self.turn = 0
        self.game_score = 0
        self.character = Coord()
        random.seed(seed)
        self.character.y = random.randint(0, self.H - 1)
        self.character.x = random.randint(0, self.W - 1)

        # init the board
        for y in range(self.H):
            for x in range(self.W):
                if y == self.character.y and x == self.character.x:
                    continue
                self.points[y][x] = random.randint(0, 9)

    def is_done(self) -> bool:
        return self.turn == self.END_TURN

    def advance(self, action: int) -> None:
        """
        action: 0, 1, 2, 3
        """
        assert not self.is_done()
        self.character.y += self.dy[action]
        self.character.x += self.dx[action]

        point = self.points[self.character.y][self.character.x]
        if point == 0:
            return

        self.game_score += point
        self.points[self.character.y][self.character.x] = 0
        self.turn += 1

    def legal_actions(self) -> list:
        actions = []
        for action in range(4):
            new_y = self.character.y + self.dy[action]
            new_x = self.character.x + self.dx[action]
            if 0 <= new_y < self.H and 0 <= new_x < self.W:
                actions.append(action)
        return actions

    def __str__(self) -> str:
        map = "\n"
        map += f"turn:\t{self.turn}\n"
        map += f"score:\t{self.game_score}\n"
        map += "=" * (self.W + 2) + "\n"
        for y in range(self.H):
            for x in range(self.W):
                if y == self.character.y and x == self.character.x:
                    map += "@"
                elif self.points[y][x] > 0:
                    map += f"{int(self.points[y][x])}"
                else:
                    map += "."
            map += "\n"
        map += "=" * (self.W + 2) + "\n"
        return map


def random_action(state: MazeState) -> int:
    random.seed(0)
    legal_actions = state.legal_actions()
    return random.choice(legal_actions)


if __name__ == "__main__":
    state = MazeState(0)
    breakpoint()
    print(state.points)
    print(state.turn)
    print(state.character)
