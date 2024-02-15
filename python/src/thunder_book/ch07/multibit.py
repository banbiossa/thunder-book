from __future__ import annotations

import copy
from typing import Literal, Optional

from bitarray import bitarray

from thunder_book.ch07 import constants as C

Bit = Literal[0, 1]


class Mat:
    def __init__(self, mat: Optional[Mat] = None) -> None:
        # self.bits is a list of length C.H
        # each is a bit int of length C.W
        if mat is None:
            self.bits: list[bitarray] = [bitarray(C.W) for _ in range(C.H)]
        else:
            self.bits = mat.bits

    def copy(self) -> Mat:
        return copy.deepcopy(self)

    def __getitem__(self, y: int, x: int):
        return self.bits[y][x]

    def __setitem__(self, y: int, x: int, value: int) -> None:
        self.bits[y][x] = value

    def remove(self, y: int, x: int) -> None:
        self.bits[y][x] = 0

    def up(self) -> Mat:
        mat = self.copy()
        for y in range(C.H - 1):
            mat.bits[y] |= mat.bits[y + 1]
        return mat

    def down(self) -> Mat:
        mat = self.copy()
        for y in range(C.H - 1, 0, -1):
            mat.bits[y] |= mat.bits[y - 1]
        return mat

    def left(self) -> Mat:
        mat = self.copy()
        for y in range(C.H):
            mat.bits[y] >>= 1
        return mat

    def right(self) -> Mat:
        mat = self.copy()
        for y in range(C.H):
            mat.bits[y] <<= 1
        return mat

    def expand(self) -> None:
        m_up = self.up()
        m_down = self.down()
        m_left = self.left()
        m_right = self.right()
        for y in range(C.H):
            self.bits[y] |= m_up.bits[y]
            self.bits[y] |= m_down.bits[y]
            self.bits[y] |= m_left.bits[y]
            self.bits[y] |= m_right.bits[y]

    def andeq_not(self, other: Mat) -> None:
        for y in range(C.H):
            self.bits[y] &= ~other.bits[y]

    def __eq__(self, other: Mat) -> bool:
        return self.bits == other.bits

    def is_any_equal(self, other: Mat) -> bool:
        for y in range(C.H):
            if self.bits[y] & other.bits[y]:
                return True
        return False
