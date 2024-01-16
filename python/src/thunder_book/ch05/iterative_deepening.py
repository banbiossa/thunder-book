import fire

from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.time_keeper import TimeKeeper
from thunder_book.ch05.alpha_beta import alpha_beta_action
from thunder_book.ch05.average_score import average_score


def iterative_deepening_action(state: State, time_threshold: int):
    time_keeper = TimeKeeper(time_threshold)
    best_action = 0
    for depth in range(1, 1_000):
        best_action = alpha_beta_action(state, depth=depth, time_keeper=time_keeper)
        if time_keeper.is_time_over():
            break
    return best_action


def compare_iterative_deepening(a: int = 30, b: int = 1):
    print(f"compare iterative deepening {a}ms vs. {b}ms")
    iterative_deepening_a = lambda state: iterative_deepening_action(state, a)
    iterative_deepening_b = lambda state: iterative_deepening_action(state, b)
    win_rate = average_score(100, (iterative_deepening_a, iterative_deepening_b))
    print(f"win rate of iterative deepening {a}ms vs. {b}ms: {win_rate:.2f}")


if __name__ == "__main__":
    fire.Fire(compare_iterative_deepening)
