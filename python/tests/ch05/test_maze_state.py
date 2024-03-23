import pytest

from thunder_book.ch05.maze_state import (
    AlternateMazeState,
    Character,
    MazeParams,
    MCTSParams,
)


@pytest.fixture
def state() -> AlternateMazeState:
    params = MazeParams(width=5, height=5, end_turn=10)
    return AlternateMazeState(0, params)


@pytest.fixture
def mcts_params():
    return MCTSParams(c=1.0, expand_threshold=10)


def test_is_done(state):
    assert not state.is_done()
    state.turn = 10
    assert state.is_done()


def test_to_string(state):
    actual = str(state)
    expected = """\
turn: 0
score(A): 0 y: 2 x: 1
score(B): 0 y: 2 x: 3

66.48
76475
9A3B8
24219
48924
"""
    assert actual == expected


def test_advance(state):
    state.advance(0)
    state.advance(0)
    actual = str(state)
    expected = """\
turn: 2
score(A): 3 y: 2 x: 2
score(B): 8 y: 2 x: 4

66.48
76475
9.A.B
24219
48924
"""
    assert actual == expected


def test_get_score(state):
    state.advance(0)
    assert state.get_score() == -3

    state.advance(0)
    assert state.get_score() == -5


def test_teban_score(state):
    assert state.teban_score() == 0.5
    state.advance(0)
    assert state.teban_score() == 0.0
    state.advance(0)
    assert state.teban_score() == 0.0


def test_legal_actions(state):
    actual = state.legal_actions()
    expected = [0, 1, 2, 3]
    assert actual == expected


def test_character_on():
    character = Character(y=1, x=2, mark="A")
    assert character.on(1, 2)
    assert not character.on(1, 3)
    assert not character.on(2, 2)


def test_white_score(state):
    assert state.teban_score() == 0.5
    assert state.white_score() == 0.5
    state.advance(0)
    assert state.teban_score() == 0.0
    assert state.white_score() == 1.0


def test_get_score_rate(state):
    assert state.get_score_rate() == 0.5
    state.advance(0)
    assert state.get_score_rate() == 0.0
    state.advance(0)
    assert 0.27 < state.get_score_rate() < 0.28
