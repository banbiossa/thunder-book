import pytest

from thunder_book.ch04.auto_move_maze_state import MazeParams, MazeState


@pytest.fixture
def state():
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    return MazeState(0, params)


def test_make_state(state):
    assert state.turn == 0
    assert state.points.shape == (5, 5)
