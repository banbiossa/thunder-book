from thunder_book.ch03.beam_search import (
    beam_search_action,
    beam_search_action_with_time_threshold,
)
from thunder_book.ch03.maze_state import MazeParams, MazeState


def test_beam_search_action():
    state = MazeState(0, MazeParams(height=3, width=4, end_turn=10))
    action = beam_search_action(state, beam_width=2, beam_depth=10)
    assert action == 3


def test_beam_search_action_with_time_threshold():
    state = MazeState(0, MazeParams(height=3, width=4, end_turn=10))
    action = beam_search_action_with_time_threshold(state, beam_width=2, time_threshold=1)
    assert action == 3
