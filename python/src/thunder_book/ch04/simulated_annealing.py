import numpy as np

from thunder_book.ch04.auto_move_maze_state import MazeState as State


def simulated_annealing(
    state: State,
    number: int,
    start_temp: float,
    end_temp: float,
):
    now_state = state.copy()
    now_state.init_characters()
    best_score = now_state.get_score()
    now_score = best_score
    best_state = now_state.copy()
    for i in range(number):
        next_state = now_state.copy()
        next_state.transition()
        next_score = next_state.get_score()
        temp = start_temp + (end_temp - start_temp) * i / number
        probability = np.exp((next_score - now_score) / temp)
        is_force_next = probability > np.random.rand()
        if (next_score > now_score) or is_force_next:
            now_state = next_state
            now_score = next_score
        if next_score > best_score:
            best_state = next_state
            best_score = next_score

    return best_state
