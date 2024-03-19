from thunder_book.ch08.maze_state import ConnectFourState, MazeParams
from thunder_book.ch08.random_action import random_action


def test_random_action():
    params = MazeParams(width=7, height=6)
    state = ConnectFourState(params)
    action = random_action(state)
    assert action in state.legal_actions()
