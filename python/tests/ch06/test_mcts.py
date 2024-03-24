import pytest

from thunder_book.ch06.maze_state import MazeParams, SimulataneousMazeState


@pytest.fixture
def state() -> SimulataneousMazeState:
    params = MazeParams(height=5, width=5, end_turn=10)
    return SimulataneousMazeState(seed=0, params=params)
