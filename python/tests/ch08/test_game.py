from thunder_book.ch08.game import play_game
from thunder_book.ch08.maze_state import MazeParams
from thunder_book.ch08.random_action import random_action


def test_play_game():
    assert (
        play_game(
            MazeParams(width=7, height=6),
            (random_action, random_action),
            should_print=True,
        )
        <= 1.0
    )
