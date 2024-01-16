import numpy as np

from thunder_book.ch05.maze_state import AlternateMazeState as State
from thunder_book.ch05.time_keeper import TimeKeeper


def alpha_beta_score(
    state: State, alpha: float, beta: float, depth: int, time_keeper: TimeKeeper
) -> float:
    if time_keeper.is_time_over():
        return 0

    if state.is_done() or depth == 0:
        return state.get_score()

    legal_actions = state.legal_actions()
    if not legal_actions:
        return state.get_score()

    for action in legal_actions:
        next_state = state.copy()
        next_state.advance(action)
        score = -alpha_beta_score(next_state, -beta, -alpha, depth - 1, time_keeper)
        if score > alpha:
            alpha = score
        if alpha >= beta:
            return alpha

    return alpha


def alpha_beta_action(
    state: State,
    depth: int,
    time_keeper: TimeKeeper = TimeKeeper(time_threshold=1_000_000_000),
) -> int:
    best_action = 0
    alpha = -np.inf
    beta = np.inf
    for action in state.legal_actions():
        next_state = state.copy()
        next_state.advance(action)
        score = -alpha_beta_score(next_state, -beta, -alpha, depth, time_keeper)
        if score > alpha:
            best_action = action
            alpha = score
        if time_keeper.is_time_over():
            break
    return best_action
