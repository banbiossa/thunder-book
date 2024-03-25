import pytest

from thunder_book.ch07.beam_search import (
    BeamType,
    beam_search_action,
    make_beam_search_f,
    play_beam_search,
    play_many_beam_search,
    time_many_beam_search,
)
from thunder_book.ch07.maze_state import (
    MazeParams,
    WallMazeState,
)


@pytest.fixture
def state() -> WallMazeState:
    params = MazeParams(height=3, width=5, end_turn=4)
    return WallMazeState(0, params)


def test_str(state):
    actual = str(state)
    expected = """\
turn: 0
score: 0

@5.#3
3###7
93524
"""

    assert actual == expected


def test_beam_search_action(state):
    actual = beam_search_action(state, 4, 10, True)
    assert actual == 2


def test_make_beam_search_f(state):
    f = make_beam_search_f(depth=4, width=10, use_zobrist_hash=False)
    assert f(state) == 2


def test_play_beam_search():
    play_beam_search()


def test_play_many_beam_search():
    play_many_beam_search(
        use_zobrist_hash=False,
        beam_type=BeamType.normal,
        num_games=1,
        width=1,
    )


def test_time_many_beam_search():
    time_many_beam_search(
        game_number=1,
        per_game=1,
        use_zobrist_hash=False,
        beam_type=BeamType.normal,
    )
