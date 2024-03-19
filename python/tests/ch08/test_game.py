from thunder_book.ch05.monte_carlo_tree_search import mcts_action
from thunder_book.ch08.game import play_black_and_white, play_game, play_many
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


def test_use_ch05():
    mcts_action_f = lambda state: mcts_action(state, 100)
    assert (
        play_game(
            MazeParams(width=7, height=6),
            (mcts_action_f, random_action),
            should_print=True,
        )
        <= 1.0
    )


def test_play_many():
    mcts_action_f = lambda state: mcts_action(state, 100)
    actual = play_many(MazeParams(width=7, height=6), (mcts_action_f, random_action), 10)
    assert actual <= 1.0


def test_play_black_and_white():
    mcts_action_f = lambda state: mcts_action(state, 100)
    actual = play_black_and_white(MazeParams(width=7, height=6), (mcts_action_f, random_action), 10)
    assert actual <= 1.0
