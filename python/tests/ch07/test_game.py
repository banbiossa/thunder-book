from thunder_book.ch07.game import BeamType, get_state, play_game, white_games
from thunder_book.ch07.maze_state import (
    MazeParams,
)
from thunder_book.ch07.numpy_state import NumpyState
from thunder_book.ch07.random_action import random_action


def test_get_state():
    state = get_state(0, BeamType.numpy, MazeParams(height=3, width=5, end_turn=4))
    assert isinstance(state, NumpyState)


def test_play_game():
    play_game(
        random_action,
        0,
        MazeParams(height=3, width=5, end_turn=4),
        beam_type=BeamType.numpy,
    )


def test_white_games():
    white_games(
        action_func=random_action,
        num_games=1,
        params=MazeParams(height=3, width=5, end_turn=4),
        beam_type=BeamType.numpy,
    )
