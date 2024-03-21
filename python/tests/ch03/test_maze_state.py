import numpy as np
import pytest

from thunder_book.ch03.maze_state import (
    Coord,
    MazeParams,
    MazeState,
    greedy_action,
    random_action,
)


@pytest.fixture
def state():
    params = MazeParams(height=3, width=4, end_turn=10)
    state = MazeState(0, params)
    return state


def test_maze_state(state):
    assert state.points.shape == (3, 4)
    assert state.turn == 0
    # if seed is working
    assert state.character.x == 3
    assert state.character.y == 1
    assert state.points.sum() != 0


def test_maze_state_str(state):
    actual = str(state)
    print(actual)
    assert isinstance(actual, str)
    expected = """\
turn: 0
score: 0

.487
647@
5938
"""
    assert actual == expected


def test_advance(state):
    state.advance(2)
    assert state.character.y == 2
    assert state.character.x == 3
    assert state.game_score == 8


def test_is_done(state):
    assert not state.is_done()
    state.turn = 10
    assert state.is_done()


def test_legal_actions(state):
    actual = state.legal_actions()
    expected = [1, 2, 3]
    assert actual == expected


def test_action_down_and_right(state):
    # if starting from (0, 0) all mazes
    # at beginning can go down(0) and right(2)
    state.character = Coord()  # 0, 0
    state.points = np.array([[0, 0], [0, 0]])
    actions = state.legal_actions()
    assert actions == [0, 2]


def test_operator(state):
    state_small = state
    state.evaluated_score = 1
    state_large = state.copy()
    state_large.evaluated_score = 2

    assert state_small < state_large
    assert state_small == state_small


def test_random_action(state):
    legal_actions = state.legal_actions()
    action = random_action(state)
    assert action in legal_actions


def test_greey_action(state):
    action = greedy_action(state)
    assert action == 2
