import pytest

from thunder_book.ch07.maze_state import (
    Character,
    MazeParams,
    State,
    WallMazeState,
    ZobristHash,
)

Character, MazeParams, State, WallMazeState, ZobristHash


@pytest.fixture
def state() -> WallMazeState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return WallMazeState(0, params)


def test_character_on():
    character = Character(y=1, x=2)
    assert character.on(1, 2)
    assert not character.on(1, 3)


def test_state(state):
    assert state.points.shape == (3, 5)
    assert state.walls.shape == (3, 5)
    assert state.turn == 0


def test_str(state):
    actual = str(state)
    expected = """\
turn: 0
score: 0

@5.#3
3###7
93524
"""
    assert actual == expected


def test_legal_actions(state):
    actual = state.legal_actions()
    expected = [0, 2]
    assert actual == expected


def test_is_done(state):
    assert not state.is_done()
    state.turn = 4
    assert state.is_done()


def test_advance(state):
    hash_before = state.hash
    state.advance(0)
    assert hash_before != state.hash

    assert state.game_score == 5


def test_get_distance_to_nearest_point(state):
    assert state.get_distance_to_nearest_point() == 1
