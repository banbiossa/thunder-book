import numpy as np

from thunder_book.ch07.maze_state import MazeParams, State


class NumpyState(State):
    def __init__(self, seed: int, params: MazeParams):
        super().__init__(seed, params)
        self.points_mat_ = np.zeros((self.params.height, self.params.width), dtype=bool)
        self.walls_mat_ = np.zeros((self.params.height, self.params.width), dtype=bool)

        for y in range(self.params.height):
            for x in range(self.params.width):
                if self.walls[y, x]:
                    self.walls_mat_[y, x] = True
                if self.points[y, x]:
                    self.points_mat_[y, x] = True

    def get_distance_to_nearest_point(self) -> int:
        mat = np.zeros((self.params.height, self.params.width), dtype=bool)
        mat[self.character.y, self.character.x] = True

        for depth in range(self.params.height * self.params.width):
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

        return self.params.height * self.params.width
