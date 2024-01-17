import fire

from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.time_keeper import TimeKeeper
from thunder_book.ch05.alpha_beta import alpha_beta_action
from thunder_book.ch05.average_score import average_score
from thunder_book.ch05 import constants


def iterative_deepening_action(state: State, time_threshold: int):
    time_keeper = TimeKeeper(time_threshold)
    best_action = 0
    # depth を大きくすると (e.g. 1000) なぜかバグる, 本来 END_TURN までやれば良いよね？
    for depth in range(1, constants.END_TURN * 2):
        best_action = alpha_beta_action(state, depth=depth, time_keeper=time_keeper)
        if time_keeper.is_time_over():
            # print(f"depth: {depth} with time {time_keeper.time_threshold}")
            break
    return best_action


def compare_iterative_deepening(a: int = 10, b: int = 1):
    print(f"compare iterative deepening {a}ms vs. {b}ms")
    iterative_deepening_a = lambda state: iterative_deepening_action(state, a)
    iterative_deepening_b = lambda state: iterative_deepening_action(state, b)
    win_rate = average_score(100, (iterative_deepening_a, iterative_deepening_b))
    print(f"win rate of iterative deepening {a}ms vs. {b}ms: {win_rate:.2f}")


if __name__ == "__main__":
    fire.Fire(compare_iterative_deepening)
