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
