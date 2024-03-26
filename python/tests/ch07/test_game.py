import pytest

from thunder_book.ch07.game import BeamType, get_state, play_game, white_games
from thunder_book.ch07.maze_state import (
    MazeParams,
)
from thunder_book.ch07.numpy_state import NumpyState

BeamType, play_game, get_state, white_games


@pytest.fixture
def state() -> NumpyState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return NumpyState(0, params)
