#include <sstream>
#include <random>
#include "maze_state.h"

AlternateMazeState::AlternateMazeState(const int seed) : points_(H, std::vector<int>(W)),
                                                         turn_(0),
                                                         characters_({
                                                             Character(H / 2, (W / 2) - 1, "A"),
                                                             Character(H / 2, (W / 2) + 1, "B"),
                                                         })
{
    auto mt_for_construct = std::mt19937(seed);
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            int point = mt_for_construct() % 10;
            if (characters_[0].y_ == y && characters_[0].x_ == x)
            {
                continue;
            }
            if (characters_[1].y_ == y && characters_[1].x_ == x)
            {
                continue;
            }
            this->points_[y][x] = point;
        }
    }
}

bool AlternateMazeState::is_done()
{
    return this->turn_ == END_TURN;
}

void AlternateMazeState::advance(const int action)
{
    auto &character = this->characters_[0];
    character.y_ += dy[action];
    character.x_ += dx[action];
    auto &point = this->points_[character.y_][character.x_];
    if (point > 0)
    {
        character.game_score_ += point;
        point = 0;
    }
    this->turn_++;
    std::swap(this->characters_[0], this->characters_[1]);
}

std::vector<int> AlternateMazeState::legal_actions() const
{
    std::vector<int> actions;
    const auto &character = this->characters_[0];
    for (int action = 0; action < 4; action++)
    {
        int ty = character.y_ + dy[action];
        int tx = character.x_ + dx[action];
        if (ty >= 0 && ty < H && tx >= 0 && tx < W)
        {
            actions.emplace_back(action);
        }
    }
    return actions;
}

void AlternateMazeState::print_end_game()
{
    using std::cout;
    using std::endl;
    if (characters_[0].game_score_ == characters_[1].game_score_)
        cout << "DRAW" << endl;
    if (characters_[0].game_score_ > characters_[1].game_score_)
        cout << "WIN " << characters_[0].mark_ << endl;
    else
        cout << "WIN " << characters_[1].mark_ << endl;
}

Character AlternateMazeState::get_winner()
{
    if (characters_[0].game_score_ > characters_[1].game_score_)
    {
        return characters_[0];
    }
    else
    {
        return characters_[1];
    }
}

WinningStatus AlternateMazeState::get_winning_status()
{
    if (is_done())
    {
        if (characters_[0].game_score_ > characters_[1].game_score_)
            return WinningStatus::WIN;
        else if (characters_[0].game_score_ < characters_[1].game_score_)
            return WinningStatus::LOSE;
        else
            return WinningStatus::DRAW;
    }
    else
        return WinningStatus::NONE;
}

std::string AlternateMazeState::to_string()
{
    std::stringstream ss("");
    ss << "turn:\t" << this->turn_ << "\n";
    // A/B start depends on turn % 2
    for (auto &character : characters_)
    {
        ss << "score(" << character.mark_ << "):\t" << character.game_score_;
        ss << "\ty:" << character.y_ << " x: " << character.x_ << "\n";
    }

    for (int h = 0; h < H; h++)
    {
        ss << "\n";
        for (int w = 0; w < W; w++)
        {
            bool is_written = false;
            for (auto &character : characters_)
            {
                if (character.y_ == h && character.x_ == w)
                {
                    ss << character.mark_;
                    is_written = true;
                }
            }
            if (!is_written)
            {
                if (this->points_[h][w] > 0)
                    ss << points_[h][w];
                else
                    ss << ".";
            }
        }
    }
    ss << "\n";
    return ss.str();
}
