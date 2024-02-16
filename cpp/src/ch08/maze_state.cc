#include "maze_state.h"

bool ConnectFourState::is_done() const
{
    return win_status_ != GameStatus::ONGOING;
}

GameStatus ConnectFourState::did_i_win() const
{
    return win_status_;
}

std::vector<int> ConnectFourState::legal_actions() const
{
    std::vector<int> actions;
    for (int x = 0; x < W; x++)
    {
        for (int y = 0; y < H; y++)
        {
            if (my_board_[y][x] == 0 && enemy_board_[y][x] == 0)
            {
                actions.emplace_back(x);
                break;
            }
        }
    }
    return actions;
}
