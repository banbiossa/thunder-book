from thunder_book.ch04.auto_move_maze_state import MazeState as State
from thunder_book.ch04.auto_move_maze_state import play_game


def hill_climb(state: State, number: int) -> State:
    now_state = state.copy()
    now_state.init_characters()

    best_score = now_state.get_score()
    for _ in range(number):
        next_state = now_state.copy()
        next_state.transition()
        next_score = next_state.get_score()
        if next_score > best_score:
            best_score = next_score
            now_state = next_state
    return now_state


if __name__ == "__main__":
    play_game(
        "hill_climb",
        lambda state: hill_climb(state, 10_000),
        42,
    )
