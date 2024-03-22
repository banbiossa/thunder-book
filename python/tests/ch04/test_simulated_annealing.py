from thunder_book.ch04.auto_move_maze_state import MazeParams, MazeState
from thunder_book.ch04.simulated_annealing import simulated_annealing


def test_simulated_annealing():
    state = MazeState(0, MazeParams(width=5, height=5, end_turn=4, num_characters=3))
    actual = simulated_annealing(state, 10, start_temp=500, end_temp=10)
    assert isinstance(actual, MazeState)
    assert state.get_score() < actual.get_score()
