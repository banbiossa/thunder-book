from thunder_book.ch03.maze_state import MazeState
from thunder_book.ch03.time_keeper import TimeKeeper


def beam_search_action(state: MazeState, beam_width: int, beam_depth: int) -> int:
    # heapq の使い勝手は悪いので　sort する
    now_beam = [state]
    best_state = state

    for t in range(beam_depth):
        next_beam = []
        for _ in range(beam_width):
            if len(now_beam) == 0:
                break
            # todo: use heapq
            now_beam.sort()
            now_state = now_beam.pop()

            legal_actions = now_state.legal_actions()
            for action in legal_actions:
                next_state = now_state.copy()
                next_state.advance(action)
                next_state.evaluate_score()
                if t == 0:
                    next_state.first_action = action
                next_beam.append(next_state)

        now_beam = sorted(next_beam)
        best_state = now_beam[-1]
        if best_state.is_done():
            break

    return best_state.first_action


def beam_search_action_with_time_threshold(
    state: MazeState, beam_width: int, time_threshold: int
) -> int:
    """_summary_

    Args:
        state (MazeState): _description_
        beam_width (int): _description_
        time_threshold (int): _description_

    Returns: action as int
    """
    time_keeper = TimeKeeper(time_threshold)

    # heapq の使い勝手は悪いので　sort する
    now_beam = [state]
    best_state = state

    while not time_keeper.is_time_over():
        next_beam = []
        for _ in range(beam_width):
            if len(now_beam) == 0:
                break
            # todo: use heapq
            now_beam.sort()
            now_state = now_beam.pop()

            legal_actions = now_state.legal_actions()
            for action in legal_actions:
                next_state = now_state.copy()
                next_state.advance(action)
                next_state.evaluate_score()
                if next_state.first_action == -1:
                    next_state.first_action = action
                next_beam.append(next_state)

        now_beam = sorted(next_beam)
        best_state = now_beam[-1]
        if best_state.is_done():
            break

    return best_state.first_action
