from thunder_book.ch03.chokudai_search import (
    chokudai_search_action,
    chokudai_search_action_with_time_threshold,
)
from thunder_book.ch03.maze_state import MazeParams, MazeState


def test_chokudai_search_action():
    state = MazeState(0, MazeParams(height=3, width=4, end_turn=10))
    actual = chokudai_search_action(state, beam_width=2, beam_depth=10, beam_number=1)
    assert actual == 3


def test_chokudai_search_action_time_threshold():
    state = MazeState(0, MazeParams(height=3, width=4, end_turn=10))
    actual = chokudai_search_action_with_time_threshold(
        state, beam_width=2, beam_depth=10, time_threshold=1
    )
    assert actual == 3
