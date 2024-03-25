from __future__ import annotations

import copy
from typing import Optional

from bitarray import bitarray

from thunder_book.ch07.maze_state import MazeParams, State


class SMat:
    def __init__(self, params: MazeParams, mat: Optional[SMat] = None) -> None:
        self.params = params
        if mat is None:
            self.bits = bitarray(self.params.width * self.params.height)
        else:
            self.bits = mat.bits

        self.left_mask = self._init_left_mask()
        self.right_mask = self._init_right_mask()

    def __setitem__(self, yx: tuple[int, int], value: int):
        y, x = yx
        self.set(y, x, value)

    def _init_left_mask(self) -> bitarray:
        mask = bitarray(self.params.width * self.params.height)
        for y in range(self.params.height):
            b1 = bitarray(self.params.height * self.params.width)
            b1[0] = 1
            mask |= b1 << (y * self.params.width)
        mask = ~mask
        return mask

    def _init_right_mask(self) -> bitarray:
        mask = bitarray(self.params.width * self.params.height)
        for y in range(self.params.height):
            b1 = bitarray(self.params.height * self.params.width)
            b1[0] = 1
            mask |= b1 << (y * self.params.width + self.params.width - 1)
        mask = ~mask
        return mask

    def copy(self) -> SMat:
        return copy.deepcopy(self)

    def up(self) -> SMat:
        mat = self.copy()
        mat.bits >>= self.params.width
        return mat

    def down(self) -> SMat:
        mat = self.copy()
        mat.bits <<= self.params.width
        return mat

    def left(self) -> SMat:
        mat = self.copy()
        mat.bits |= (mat.bits & self.left_mask) >> 1
        return mat

    def right(self) -> SMat:
        mat = self.copy()
        mat.bits != (mat.bits & self.right_mask) << 1
        return mat

    def get(self, y: int, x: int) -> int:
        return self.bits[y * self.params.height + x]

    def set(self, y: int, x: int, value: int) -> None:
        self.bits[y * self.params.height + x] = value

    def remove(self, y: int, x: int) -> None:
        self.bits[y * self.params.height + x] = 0

    def expand(self) -> None:
        self.bits |= self.up().bits
        self.bits |= self.down().bits
        self.bits |= self.left().bits
        self.bits |= self.right().bits

    def andeq_not(self, other: SMat) -> None:
        self.bits &= ~other.bits

    def is_equal(self, other: SMat) -> bool:
        return self.bits == other.bits

    def is_any_equal(self, other: SMat) -> bool:
        return any(self.bits & other.bits)


class SinglebitState(State):
    def __init__(self, seed: int, params: MazeParams):
        super().__init__(seed, params)
        self.points_mat_ = SMat(params)
        self.walls_mat_ = SMat(params)

        for y in range(self.params.height):
            for x in range(self.params.width):
                if self.walls[y, x]:
                    self.walls_mat_[y, x] = 1
                if self.points[y, x]:
                    self.points_mat_[y, x] = 1

    def get_distance_to_nearest_point(self) -> int:
        mat = SMat(self.params)
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
