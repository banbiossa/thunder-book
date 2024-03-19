from thunder_book.ch05.monte_carlo_tree_search import mcts_action
from thunder_book.ch08.bitset import ConnectFourBitset
from thunder_book.ch08.game import play_black_and_white, play_game, play_many
from thunder_book.ch08.maze_state import ConnectFourState, MazeParams
from thunder_book.ch08.random_action import random_action


def test_play_game():
    assert (
        play_game(
            MazeParams(width=7, height=6),
            (random_action, random_action),
            state_types=(ConnectFourState, ConnectFourState),
            should_print=True,
        )
        <= 1.0
    )


def test_play_game_bit():
    assert (
        play_game(
            MazeParams(width=7, height=6),
            (random_action, random_action),
            state_types=(ConnectFourState, ConnectFourBitset),
            should_print=True,
        )
        <= 1.0
    )


def test_use_ch05():
    mcts_action_f = lambda state: mcts_action(state, 10)
    assert (
        play_game(
            MazeParams(width=7, height=6),
            (mcts_action_f, random_action),
            state_types=(ConnectFourState, ConnectFourState),
            should_print=True,
        )
        <= 1.0
    )


def test_play_many():
    mcts_action_f = lambda state: mcts_action(state, 100)
    actual = play_many(
        MazeParams(width=7, height=6),
        (mcts_action_f, random_action),
        state_types=(ConnectFourState, ConnectFourState),
        num_games=2,
    )
    assert actual <= 1.0


def test_play_black_and_white():
    mcts_action_f = lambda state: mcts_action(state, 100)
    actual = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_action_f, random_action),
        state_types=(ConnectFourState, ConnectFourState),
        num_games=2,
    )
    assert actual <= 1.0


def test_play_black_and_white_bit():
    mcts_action_f = lambda state: mcts_action(state, 100)
    actual = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_action_f, random_action),
        state_types=(ConnectFourState, ConnectFourBitset),
        num_games=2,
    )
    assert actual <= 1.0
