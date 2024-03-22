from thunder_book.ch04.average_score import run, run_many
from thunder_book.ch04.hill_climb import hill_climb


def test_run_many():
    actual = run_many(lambda state: hill_climb(state, 10), 10)
    assert actual > 0


def test_run():
    run(num_games=1, num_simulate=1)
