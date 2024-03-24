import pytest

from thunder_book.ch06.maze_state import Character, MazeParams, SimulataneousMazeState


@pytest.fixture
def state() -> SimulataneousMazeState:
    params = MazeParams(height=5, width=5, end_turn=10)
    return SimulataneousMazeState(seed=0, params=params)


def test_character_on():
    character = Character(y=1, x=2, mark="A")
    assert character.on(1, 2)
    assert not character.on(1, 3)
    assert not character.on(2, 2)


def test_is_done(state):
    assert not state.is_done()
    state.turn = 10
    assert state.is_done()


def test_score(state):
    assert state.score(0) == 0.5
    assert state.score(1) == 0.5


def test_str(state):
    actual = str(state)
    expected = """\
turn: 0
score(A): 0 y: 2 x: 1
score(B): 0 y: 2 x: 3

84.48
57475
4A8B4
84948
11411
"""
    assert actual == expected


def test_get_char(state):
    assert state._get_char(2, 1) == "A"
    assert state._get_char(2, 3) == "B"
    assert state._get_char(0, 2) == "."
    assert state._get_char(0, 0) == "8"

    state.characters[0].x = 3
    assert state._get_char(2, 3) == "X"


def test_legal_actions(state):
    assert state.legal_actions(0) == [0, 1, 2, 3]
    assert state.legal_actions(1) == [0, 1, 2, 3]

    state.characters[0].x = 0
    actual = str(state)
    expected = """\
turn: 0
score(A): 0 y: 2 x: 0
score(B): 0 y: 2 x: 3

84.48
57475
A.8B4
84948
11411
"""
    assert actual == expected
    assert state.legal_actions(0) == [0, 2, 3]
