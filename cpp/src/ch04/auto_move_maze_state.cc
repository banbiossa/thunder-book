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
    auto board_chars = std::vector<std::vector<char>>(H, std::vector<char>(W, '.'));
    for (int h = 0; h < H; h++)
    {
        for (int w = 0; w < W; w++)
        {
            bool is_written = false; // この座標に書く文字が決定したか

            for (const auto &character : this->characters_)
            {
                if (character.y_ == h && character.x_ == w)
                {
                    ss << "@";
                    is_written = true;
                    break;
                }
                board_chars[character.y_][character.x_] = '@';
            }
            if (!is_written)
            {
                if (this->points_[h][w] > 0)
                {
                    ss << points_[h][w];
                }
                else
                {
                    ss << '.';
                }
            }
        }
        ss << '\n';
    }

    return ss.str();
}
