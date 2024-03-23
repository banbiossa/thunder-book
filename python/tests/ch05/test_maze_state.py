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


def test_character_on():
    character = Character(y=1, x=2, mark="A")
    assert character.on(1, 2)
    assert not character.on(1, 3)
    assert not character.on(2, 2)
