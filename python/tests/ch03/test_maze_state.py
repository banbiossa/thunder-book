import pytest

import numpy as np

from thunder_book.ch03.maze_state import MazeState, Coord


def test_maze_state():
    state = MazeState(0)
    assert state.points.shape == (3, 4)
    assert state.turn == 0
    # if seed is working
    assert state.character.x == 3
    assert state.character.y == 1
    assert state.points.sum() != 0


def test_maze_state_str():
    state = MazeState(1)
    map = str(state)
    print(map)
    assert isinstance(map, str)


def test_action_down_and_right():
    # if starting from (0, 0) all mazes
    # at beginning can go down(0) and right(2)
    state = MazeState(0, 2, 2, 2)
    state.character = Coord()  # 0, 0
    state.points = np.array([[0, 0], [0, 0]])
    actions = state.legal_actions()
    assert actions == [0, 2]

    state = MazeState(0)
    state.character = Coord()  # 0, 0
    state.points = np.array([[0, 0], [0, 0]])
    actions = state.legal_actions()
    assert actions == [0, 2]
