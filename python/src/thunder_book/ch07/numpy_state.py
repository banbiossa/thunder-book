import numpy as np

from thunder_book.ch07 import constants as C
from thunder_book.ch07.maze_state import State


class NumpyState(State):
    def __init__(self, seed: int):
        super().__init__(seed)
        self.points_mat_ = np.zeros((C.H, C.W), dtype=bool)
        self.walls_mat_ = np.zeros((C.H, C.W), dtype=bool)

        for y in range(C.H):
            for x in range(C.W):
                if self.walls[y, x]:
                    self.walls_mat_[y, x] = True
                if self.points[y, x]:
                    self.points_mat_[y, x] = True

    def get_distance_to_nearest_point(self) -> int:
        mat = np.zeros((C.H, C.W), dtype=bool)
        mat[self.character.y, self.character.x] = True

        for depth in range(C.H * C.W):
            if np.any(mat & self.points_mat_):
                return depth

            next = mat.copy()
            next[1:, :] |= mat[:-1, :]
            next[:-1, :] |= mat[1:, :]
            next[:, 1:] |= mat[:, :-1]
            next[:, :-1] |= mat[:, 1:]
            next &= ~self.walls_mat_
            if np.all(next == mat):
                break
            mat = next

        return C.H * C.W
