import pytest

from thunder_book.ch08.maze_state import ConnectFourState, D, MazeParams, Status, Stone


@pytest.fixture
def state():
    params = MazeParams(width=7, height=6)
    state = ConnectFourState(params)
    return state


def test_maze_state(state):
    assert state.is_done() is False
    assert state.status == Status.ONGOING


def test_legal_actions(state):
    legal_actions = state.legal_actions()
    assert legal_actions == [0, 1, 2, 3, 4, 5, 6]


def test_to_string(state):
    actual = state.to_string()
    expected = """is_first: True

.......
.......
.......
.......
.......
.......
"""
    assert actual == expected


def test_place_stone(state):
    stone = state.place_stone(0)
    assert stone == Stone(x=0, y=0)


def test_teban_score(state):
    assert state.teban_score() == 0.5


def test_check_connection(state):
    assert not state.check_connection(Stone(x=0, y=0), D.UP, D.STAY)


def test_advance(state):
    state.advance(0)
    assert state.status == Status.ONGOING
    assert state.is_first is False
    # 入れ替わっている
    assert not state.my_board[0][0]
    assert state.enemy_board[0][0]
    assert not state.my_board[5][0]
    assert not state.enemy_board[5][0]
    assert state.teban_score() == 0.5
    assert state.legal_actions() == [0, 1, 2, 3, 4, 5, 6]
    assert (
        state.to_string()
        == """is_first: False

.......
.......
.......
.......
.......
X......
"""
    )


def test_advance_many(state):
    state.advance(0)
    state.advance(1)
    state.advance(0)
    state.advance(1)
    state.advance(0)
    state.advance(1)
    assert not state.is_done()
    state.advance(0)
    assert state.is_done()
    assert state.white_score() == 1.0
    assert state.teban_score() == 0.0
    expected = """is_first: False

.......
.......
X......
XO.....
XO.....
XO.....
"""
    assert expected == state.to_string()
