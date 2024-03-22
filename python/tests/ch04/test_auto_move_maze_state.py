import copy

import pytest

from thunder_book.ch04.auto_move_maze_state import Coord, MazeParams, MazeState


@pytest.fixture
def state():
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    t_state = MazeState(1, params)
    t_state.init_characters()
    return t_state


def test_make_state(state):
    assert state.turn == 0
    assert state.points.shape == (5, 5)
    assert state.points.max() <= 9


def test_str(state):
    actual = str(state)
    expected = """\
turn: 0
score: 0

291@@
777@3
17.66
9.743
915..
"""
    assert actual == expected


def test_init_characters(state):
    assert state.characters[0] == Coord(x=4, y=0)
    assert state.characters[1] == Coord(x=3, y=0)
    assert state.characters[2] == Coord(x=3, y=1)


def test_set_character(state):
    assert state.characters[0] == Coord(x=4, y=0)
    state.set_character(0, 1, 1)
    assert state.characters[0] == Coord(x=1, y=1)


def test_transition(state):
    before = copy.deepcopy(state.characters)
    state.transition()
    # todo: could be flaky
    assert state.characters != before
    assert sum([a == b for (a, b) in zip(state.characters, before)]) == 2
    assert state.characters[2] == Coord(x=4, y=0)


def test_get_score(state):
    actual = state.get_score(should_print=True)
    assert actual == 53.0

    state.transition()
    actual = state.get_score(should_print=True)
    assert actual == 48.0

    # assert False


def test_is_done(state):
    assert not state.is_done()
    state.turn = 4
    assert state.is_done()


def test_copy(state):
    before = state.copy()
    state.turn = 1
    assert before.turn == 0


def test_move_player(state):
    assert state.characters[0] == Coord(x=4, y=0)
    # 実際には advance の前に初期化するが単体テストではそれを再現できないので、
    # ここでplayer 上のpointを消しておく
    # (じゃないと player 上に移動して直感的でない)
    for character in state.characters:
        state.points[character.y][character.x] = 0
    state.move_player(0)
    assert state.characters[0] == Coord(x=4, y=1)


def test_advance(state):
    # 実際には advance の前に初期化するが単体テストではそれを再現できないので、
    # ここでplayer 上のpointを消しておく
    # (じゃないと player 上に移動して直感的でない)
    for character in state.characters:
        state.points[character.y][character.x] = 0
    state.advance()
    assert state.turn == 1
    assert state.game_score == 11.0
    assert state.characters[0] == Coord(x=4, y=1)
    assert state.characters[1] == Coord(x=2, y=0)
    assert state.characters[2] == Coord(x=2, y=1)
