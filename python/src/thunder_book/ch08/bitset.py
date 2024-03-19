from __future__ import annotations

import copy

from thunder_book.ch08.maze_state import MazeParams, MazeState, Status


class ConnectFourBitset(MazeState):
    """ConnectFourState と同じように実装しておく"""

    def __init__(self, params: MazeParams) -> None:
        self.my_board = 0
        self.all_board = 0
        self.is_first = True
        self.status = Status.ONGOING
        self.params = params

    def copy(self) -> ConnectFourBitset:
        return copy.deepcopy(self)

    def is_done(self) -> bool:
        return self.status != Status.ONGOING

    def get_status(self) -> Status:
        return self.status

    def legal_actions(self) -> list[int]:
        actions = []
        possible = self.all_board + self.floor_bit()
        # filter is 0b011111 [lenght of height]
        for x in range(self.params.width):
            filter = self.filter(x)
            if (filter & possible) != 0:
                actions.append(x)

        return actions

    def advance(self, action: int) -> None:
        # 視点の切り替え
        self.my_board ^= self.all_board
        self.is_first = not self.is_first

        # action の加算
        action_as_floor_bit = 1 << (action * (self.params.height + 1))
        self.all_board |= self.all_board + action_as_floor_bit

        # 終了判定
        if self.is_winner(self.my_board ^ self.all_board):
            self.status = Status.LOSE
        elif self.all_board == self.filled():
            self.status = Status.DRAW

    def floor_bit(self) -> int:
        # 0b00000010000001...
        # w: 繰り返しの数 h: 0 の数
        bit = 0
        for x in range(self.params.width):
            bit |= 1 << (x * (self.params.height + 1))
        return bit

    def filled(self) -> int:
        """
        0b01111110111111...
        makes a head missing bit, this means all board is filled

        h = 2 として
        1 << h = 100
        100 - 1 = 011
        011 << x*(h+1) = 011000
        の組み合わせ
        """
        bit = 0
        for x in range(self.params.width):
            t = 1 << self.params.height
            t -= 1
            t <<= x * (self.params.height + 1)
            bit |= t
        return bit

    def filter(self, w: int) -> int:
        # 高さ全部が 1 になるようなビット 0b011
        # 0b011 とか 0b011000 とか
        bits = (1 << self.params.height) - 1
        width = (self.params.height + 1) * w
        return bits << width

    def is_winner(self, board: int) -> bool:
        # 横方向
        t = board & (board >> (self.params.height + 1))
        if t & (t >> (2 * (self.params.height + 1))):
            return True

        # 斜め方向
        t = board & (board >> self.params.height)
        if t & (t >> (2 * self.params.height)):
            return True

        t = board & (board >> (self.params.height + 2))
        if t & (t >> (2 * (self.params.height + 2))):
            return True

        # 縦方向
        t = board & (board >> 1)
        if t & (t >> 2):
            return True

        return False

    def teban_score(self) -> float:
        match self.status:
            case Status.WIN:
                return 1.0
            case Status.LOSE:
                return 0.0
            case Status.DRAW:
                return 0.5
            case _:
                # should not reach here
                return 0.5

    def white_score(self) -> float:
        score = self.teban_score()
        if not self.is_first:
            return 1 - score
        return score
