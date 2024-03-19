import pytest

from thunder_book.ch08.bitset import ConnectFourBitset
from thunder_book.ch08.maze_state import MazeParams, Status


@pytest.fixture
def state():
    return ConnectFourBitset(MazeParams(width=4, height=2))


def test_bitset(state):
    assert not state.is_done()
    assert state.my_board == 0


def test_floor_bit(state):
    actual = state.floor_bit()
    expected = 0b001001001001
    assert actual == expected


def test_filled(state):
    actual = state.filled()
    expected = 0b011011011011
    assert actual == expected


def test_filter(state):
    actual = state.filter(0)
    expected = 0b011
    assert actual == expected

    actual = state.filter(2)
    expected = 0b011000000
    assert actual == expected


def test_legal_actions(state):
    actual = state.legal_actions()
    expected = [0, 1, 2, 3]
    assert actual == expected


def test_is_winner(state):
    board = 0b001001001001
    actual = state.is_winner(board)
    assert actual

    board = 0b001001001000
    actual = state.is_winner(board)
    assert not actual


def test_is_winner_large():
    state = ConnectFourBitset(MazeParams(width=7, height=6))
    board = 0b0000001000000100000010000001000000100000010000001
    actual = state.is_winner(board)
    assert actual

    board = 0b0000001000000100000010000000000000100000010000001
    actual = state.is_winner(board)
    assert not actual

    board = 0b0000001000001000001000001000000000100000010000001
    actual = state.is_winner(board)
    assert actual

    board = 0b0000001000001000001000010000000000100000010000001
    actual = state.is_winner(board)
    assert not actual


def test_advance(state):
    state.advance(0)
    state.advance(0)
    state.advance(1)
    state.advance(1)
    state.advance(2)
    state.advance(2)
    assert not state.is_done()
    state.advance(3)
    assert state.is_done()
    assert state.status == Status.LOSE
    assert state.white_score() == 1.0
    assert state.teban_score() == 0.0
