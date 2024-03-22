from thunder_book.ch04.auto_move_maze_state import MazeParams, MazeState
from thunder_book.ch04.hill_climb import hill_climb


def test_hill_climb():
    params = MazeParams(width=5, height=5, end_turn=4, num_characters=3)
    state = MazeState(0, params)
    actual = hill_climb(state, 10)
    assert isinstance(actual, MazeState)
    assert state.get_score() < actual.get_score()
