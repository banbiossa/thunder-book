from thunder_book.ch06.game import (
    games_black_and_white,
    many_games,
    one_game,
    play_game,
)
from thunder_book.ch06.random_action import random_action


def test_play_game():
    play_game(random_action, random_action, seed=0)


def test_one_game():
    one_game((random_action, random_action), seed=0, player_id=0)


def test_many_games():
    many_games(10, (random_action, random_action), player_id=0, print_every=10)


def test_games_black_and_white():
    games_black_and_white(10, (random_action, random_action), print_every=10)
