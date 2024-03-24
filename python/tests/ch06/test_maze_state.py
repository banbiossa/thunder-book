import pytest

from thunder_book.ch06.maze_state import Character, SimulataneousMazeState


@pytest.fixture
def state() -> SimulataneousMazeState:
    return SimulataneousMazeState(seed=0)


def test_character_on():
    character = Character(y=1, x=2, mark="A")
    assert character.on(1, 2)
    assert not character.on(1, 3)
    assert not character.on(2, 2)
