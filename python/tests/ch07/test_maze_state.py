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
