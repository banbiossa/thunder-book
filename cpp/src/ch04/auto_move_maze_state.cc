#include <sstream>
#include <random>
#include "auto_move_maze_state.h"

AutoMoveMazeState::AutoMoveMazeState(const int seed) : turn_(0),
                                                       game_score_(0),
                                                       evaluated_score_(0)
{
    auto mt_for_construct = std::mt19937(seed);
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            points_[y][x] = mt_for_construct() % 9 + 1;
        }
    }
}

void AutoMoveMazeState::set_character(const int character_id,
                                      const int y,
                                      const int x)
{
    this->characters_[character_id].y_ = y;
    this->characters_[character_id].x_ = x;
}

bool AutoMoveMazeState::is_done() const
{
    return this->turn_ == END_TURN;
}

std::string AutoMoveMazeState::to_string() const
{
    std::stringstream ss;
    ss << "turn:\t" << this->turn_ << "\n";
    ss << "score:\t" << this->game_score_ << "\n";
    for (int h = 0; h < H; h++)
    {
        for (int w = 0; w < W; w++)
        {
            for (Coord character_ : characters_)
            {
                if (character_.y_ == h && character_.x_ == w)
                {
                    ss << '@';
                }
            }
            if (this->points_[h][w] > 0)
            {
                ss << points_[h][w];
            }
            else
            {
                ss << '.';
            }
        }
        ss << "\n";
    }
    return ss.str();
}
