from __future__ import annotations

import copy
from typing import Optional

from bitarray import bitarray

from thunder_book.ch07.maze_state import MazeParams, State


class Mat:
    def __init__(self, params: MazeParams, mat: Optional[Mat] = None) -> None:
        # self.bits is a list of length self.params.height
        # each is a bit int of length self.params.width
        self.params = params
        if mat is None:
            self.bits: list[bitarray] = [
                bitarray(self.params.width) for _ in range(self.params.height)
            ]
        else:
            self.bits = mat.bits

    def copy(self) -> Mat:
        return copy.deepcopy(self)

    def __getitem__(self, yx: tuple[int, int]):
        y, x = yx
        return self.bits[y][x]

    def __setitem__(self, yx: tuple[int, int], value: int):
        y, x = yx
        self.bits[y][x] = value

    def up(self) -> Mat:
        mat = self.copy()
        for y in range(self.params.height - 1):
            mat.bits[y] |= mat.bits[y + 1]
        return mat

    def down(self) -> Mat:
        mat = self.copy()
        for y in range(self.params.height - 1, 0, -1):
            mat.bits[y] |= mat.bits[y - 1]
        return mat

    def left(self) -> Mat:
        mat = self.copy()
        for y in range(self.params.height):
            mat.bits[y] >>= 1
        return mat

    def right(self) -> Mat:
        mat = self.copy()
        for y in range(self.params.height):
            mat.bits[y] <<= 1
        return mat

    def expand(self) -> None:
        m_up = self.up()
        m_down = self.down()
        m_left = self.left()
        m_right = self.right()
        for y in range(self.params.height):
            self.bits[y] |= m_up.bits[y]
            self.bits[y] |= m_down.bits[y]
            self.bits[y] |= m_left.bits[y]
            self.bits[y] |= m_right.bits[y]

    def andeq_not(self, other: Mat) -> None:
        for y in range(self.params.height):
            self.bits[y] &= ~other.bits[y]

    def __eq__(self, other: Mat) -> bool:
        return self.bits == other.bits

    def is_any_equal(self, other: Mat) -> bool:
        for y in range(self.params.height):
            if self.bits[y] & other.bits[y]:
                return True
        return False


class MultibitState(State):
    def __init__(self, seed: int, params: MazeParams):
        super().__init__(seed, params)
        self.walls_mat_ = Mat(params)
        self.points_mat_ = Mat(params)

        for y in range(self.params.height):
            for x in range(self.params.width):
                if self.walls[y, x]:
                    self.walls_mat_[y, x] = 1
                if self.points[y, x]:
                    self.points_mat_[y, x] = 1

    def get_distance_to_nearest_point(self) -> int:
        mat = Mat(self.params)
        mat[self.character.y, self.character.x] = 1

        for depth in range(self.params.height * self.params.width):
            if mat.is_any_equal(self.points_mat_):
                return depth

            next = mat.copy()
            next.expand()
            next.andeq_not(self.walls_mat_)
            if next == mat:
                break
            mat = next

        return self.params.height * self.params.width
