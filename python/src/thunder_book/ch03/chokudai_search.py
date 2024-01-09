from thunder_book.ch03.maze_state import MazeState
from thunder_book.ch03.time_keeper import TimeKeeper


def chokudai_search_action(
    state: MazeState,
    beam_width: int,
    beam_depth: int,
    beam_number: int,
) -> int:
    """_summary_

    Args:
        state (MazeState): _description_
        beam_width (int): _description_
        beam_depth (int): _description_
        beam_number (int): _description_

    Returns:
        int: action
    """
    beam = [[] for _ in range(beam_depth + 1)]
    beam[0].append(state)
    # beams
    for _ in range(beam_number):
        # depth
        for t in range(beam_depth):
            now_beam = beam[t]
            next_beam = beam[t + 1]

            # width
            for i in range(beam_width):
                if len(now_beam) == 0:
                    break
                now_beam.sort()
                now_state = now_beam[-1]
                if now_state.is_done():
                    break
                now_beam = now_beam[:-1]

                # actions
                legal_actions = now_state.legal_actions()
                for action in legal_actions:
                    next_state = now_state.copy()
                    next_state.advance(action)
                    next_state.evaluate_score()
                    if t == 0:
                        next_state.first_action = action
                    next_beam.append(next_state)
    for t in range(beam_depth, -1, -1):
        now_beam = beam[t]
        if len(now_beam) != 0:
            now_beam.sort()
            return now_beam[-1].first_action
    return -1


def chokudai_search_action_with_time_threshold(
    state: MazeState,
    beam_width: int,
    beam_depth: int,
    time_threshold: int,
) -> int:
    """_summary_

    Args:
        state (MazeState): _description_
        beam_width (int): _description_
        beam_depth (int): _description_
        time_threshold (int): _description_

    Returns:
        int: action
    """
    time_keeper = TimeKeeper(time_threshold)
    beam = [[] for _ in range(beam_depth + 1)]
    beam[0].append(state)
    # beams
    while not time_keeper.is_time_over():
        # depth
        for t in range(beam_depth):
            now_beam = beam[t]
            next_beam = beam[t + 1]

            # width
            for i in range(beam_width):
                if len(now_beam) == 0:
                    break
                now_beam.sort()
                now_state = now_beam[-1]
                if now_state.is_done():
                    break
                now_beam = now_beam[:-1]

                # actions
                legal_actions = now_state.legal_actions()
                for action in legal_actions:
                    next_state = now_state.copy()
                    next_state.advance(action)
                    next_state.evaluate_score()
                    if t == 0:
                        next_state.first_action = action
                    next_beam.append(next_state)
    for t in range(beam_depth, -1, -1):
        now_beam = beam[t]
        if len(now_beam) != 0:
            now_beam.sort()
            return now_beam[-1].first_action
    return -1


if __name__ == "__main__":
    state = MazeState(0)
    while not state.is_done():
        action = chokudai_search_action(state, 2, 3, 3)
        state.advance(action)
    print(state.game_score)
