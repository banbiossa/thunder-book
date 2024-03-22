from thunder_book.ch04.average_score import run_many
from thunder_book.ch04.hill_climb import hill_climb


def test_run_many():
    actual = run_many(lambda state: hill_climb(state, 10), 10)
    assert actual > 0
