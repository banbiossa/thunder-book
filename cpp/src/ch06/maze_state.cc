#include <random>
#include "maze_state.h"

SimultaneousMazeState::SimultaneousMazeState(const int seed) : points_(H, std::vector<int>(W)),
                                                               turn_(0),
                                                               characters_(
                                                                   {Character(H / 2, (W / 2) - 1),
                                                                    Character(H / 2, (W / 2) + 1)})
{
    auto mt_for_construct = std::mt19937(seed);
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            int ty = y;
            int tx = x;
            int point = mt_for_construct() % 10;
            if (characters_[0].y_ == y && characters_[0].x_ == x)
                continue;
            if (characters_[1].y_ == y && characters_[1].x_ == x)
                continue;
            this->points_[ty][tx] = point;
            // make it symmetrical
            tx = W - 1 - x;
            this->points_[ty][tx] = point;
        }
    }
}

bool SimultaneousMazeState::is_done()
{
    return turn_ == END_TURN;
}

void SimultaneousMazeState::advance(const int action0, const int action1)
{
    {
        auto &character = this->characters_[0];
        const auto &action = action0;
        character.y_ += dy[action];
        character.x_ += dx[action];
        const auto point = this->points_[character.y_][character.x_];
        if (point > 0)
            character.game_score_ += point;
    }
}
