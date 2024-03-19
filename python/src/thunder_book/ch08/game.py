from thunder_book.ch05.monte_carlo_tree_search import mcts_action
from thunder_book.ch08.maze_state import ActionFunc, ConnectFourState, MazeParams
from thunder_book.ch08.random_action import random_action


def play_game(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    should_print: bool = False,
) -> float:
    state = ConnectFourState(params)

    if should_print:
        print(state.to_string())

    player = 0
    while not state.is_done():
        state.advance(actions[player](state))
        player ^= 1
        if should_print:
            print(state.to_string())

    return state.white_score()


def play_many(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    num_games: int,
    print_every: int = 10,
) -> float:
    total_score = 0.0
    for i in range(num_games):
        total_score += play_game(params, actions)

        if print_every > 0 and (i + 1) % print_every == 0:
            print(f"{i+1} {total_score / (i+1):.2f}")
    return total_score / num_games


def play_black_and_white(
    params: MazeParams,
    actions: tuple[ActionFunc, ActionFunc],
    num_games: int,
    print_every: int = 10,
) -> float:
    total = play_many(params, actions, num_games, print_every)
    actions_bw = (actions[1], actions[0])
    total += 1 - play_many(params, actions_bw, num_games, print_every)
    return total / 2


if __name__ == "__main__":
    mcts_action_f = lambda state: mcts_action(state, 100)
    score = play_black_and_white(
        MazeParams(width=7, height=6),
        (mcts_action_f, random_action),
        100,
    )
    print(score)
