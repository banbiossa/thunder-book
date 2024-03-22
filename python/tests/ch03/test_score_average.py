from thunder_book.ch03.maze_state import MazeParams, random_action
from thunder_book.ch03.score_average import play_many, play_one


def test_play_one():
    params = MazeParams(height=3, width=4, end_turn=10)
    score, time = play_one(2, random_action, params)
    assert 0 <= score
    assert time > 0


def test_play_many():
    play_many(1)
