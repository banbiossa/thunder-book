from enum import Enum

from thunder_book.ch07.maze_state import ActionFunc, MazeParams, State, WallMazeState
from thunder_book.ch07.multibit import MultibitState
from thunder_book.ch07.numpy_state import NumpyState
from thunder_book.ch07.singlebit import SinglebitState

ACTION_TO_STR = ["RIGHT", "LEFT", "DOWN", "UP"]


class BeamType(str, Enum):
    normal = "normal"
    multi = "multi"
    single = "single"
    numpy = "numpy"


def get_state(seed: int, beam_type: BeamType, params: MazeParams) -> State:
    match beam_type:
        case BeamType.normal:
            return WallMazeState(seed, params)
        case BeamType.multi:
            return MultibitState(seed, params)
        case BeamType.single:
            return SinglebitState(seed, params)
        case BeamType.numpy:
            return NumpyState(seed, params)


def play_game(
    action_func: ActionFunc,
    seed: int,
    params: MazeParams,
    beam_type: BeamType,
) -> None:
    state = get_state(seed, beam_type, params)
    print(state)
    while not state.is_done():
        action = action_func(state)
        print(f"action: {ACTION_TO_STR[action]}")
        state.advance(action)
        print(state)


def white_games(
    action_func: ActionFunc,
    params: MazeParams,
    num_games: int,
    beam_type: BeamType,
    print_every: int = 10,
) -> float:
    total = 0
    for i in range(num_games):
        state = get_state(i, beam_type, params)
        while not state.is_done():
            action = action_func(state)
            state.advance(action)
        total += state.game_score
        if i % print_every == 0:
            print(f"game {i} score: {total/(i+1):.1f}")

    return total / num_games
