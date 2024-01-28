import fire

from thunder_book.ch06 import constants as C
from thunder_book.ch06.maze_state import ActionFunc
from thunder_book.ch06.maze_state import SimulataneousMazeState as State
from thunder_book.ch06.random_action import random_action


def play_game(action_f0: ActionFunc, action_f1: ActionFunc, seed: int):
    state = State(seed=seed)
    print(state)

    while not state.is_done():
        action0 = action_f0(state, 0)
        action1 = action_f1(state, 1)
        print(f"actions {C.dtor[action0]} {C.dtor[action1]}")
        state.advance(action0, action1)
        print(state)


def one_game(
    actions: tuple[ActionFunc, ActionFunc], seed: int, player_id: int
) -> float:
    state = State(seed=seed)
    while not state.is_done():
        action0 = actions[0](state, 0)
        action1 = actions[1](state, 1)
        state.advance(action0, action1)
    return state.score(player_id)


def many_games(
    num_games: int,
    actions_wb: tuple[ActionFunc, ActionFunc],
    player_id: int,
    print_every: int,
) -> float:
    score = 0
    for i in range(num_games):
        score += one_game(actions_wb, seed=i, player_id=player_id)

        if i % print_every == 0:
            tmp = score / (i + 1)
            print(f"{i=} {tmp=:.2f}")

    return score / num_games


def games_black_and_white(
    num_gams: int, actions_wb: tuple[ActionFunc, ActionFunc], print_every: int = 10
):
    print("play white")
    score = many_games(num_gams, actions_wb, player_id=0, print_every=print_every)

    print()

    print("play black")
    actions_bw = (actions_wb[1], actions_wb[0])
    score += many_games(num_gams, actions_bw, player_id=1, print_every=print_every)

    print()

    return score / 2


def random_vs_random():
    play_game(random_action, random_action, seed=0)


if __name__ == "__main__":
    fire.Fire(random_vs_random)
