import numpy as np

from thunder_book.ch05.maze_state import AlternateMazeState as State


def mini_max_score(state: State, depth: int) -> float:
    if state.is_done() or depth == 0:
        return state.get_score()

    legal_actions = state.legal_actions()
    if not legal_actions:
        return state.get_score()

    best_score = -np.inf
    for action in legal_actions:
        next_state = state.copy()
        next_state.advance(action)
        score = -mini_max_score(next_state, depth - 1)
        if score > best_score:
            best_score = score
    return best_score


def mini_max_action(state: State, depth: int) -> int:
    best_action = -1
    best_score = -np.inf
    for action in state.legal_actions():
        next_state = state.copy()
        next_state.advance(action)
        score = -mini_max_score(next_state, depth)
        if score > best_score:
            best_action = action
            best_score = score
    return best_action
