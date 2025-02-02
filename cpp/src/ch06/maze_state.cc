#include <random>
#include <sstream>
#include "maze_state.h"

SimultaneousMazeState::SimultaneousMazeState(
    const int seed) : points_(H, std::vector<int>(W)),
                      turn_(0),
                      characters_(
                          {Character(H / 2, (W / 2) - 1, "A"),
                           Character(H / 2, (W / 2) + 1, "B")})
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

void SimultaneousMazeState::_advance(const int player_id, const int action)
{
    auto &character = this->characters_[player_id];
    character.y_ += dy[action];
    character.x_ += dx[action];
    const auto point = this->points_[character.y_][character.x_];
    if (point > 0)
        character.game_score_ += point;
}

void SimultaneousMazeState::advance(const int action0, const int action1)
{
    this->_advance(0, action0);
    this->_advance(1, action1);
    for (const auto &character : this->characters_)
    {
        this->points_[character.y_][character.x_] = 0;
    }
    this->turn_++;
}

std::vector<int> SimultaneousMazeState::legal_actions(const int player_id) const
{
    std::vector<int> actions;
    const auto &character = this->characters_[player_id];
    for (int action = 0; action < 4; action++)
    {
        int ty = character.y_ + dy[action];
        int tx = character.x_ + dx[action];
        if (ty >= 0 && ty < H && tx >= 0 && tx < W)
            actions.emplace_back(action);
    }
    return actions;
}

std::string SimultaneousMazeState::to_string()
{
    std::stringstream ss("");
    ss << "\nturn: " << this->turn_ << "\n";
    // A/B start depends on turn % 2
    for (auto &character : characters_)
    {
        ss << "score(" << character.mark_ << "): " << character.game_score_;
        ss << " y:" << character.y_ << " x: " << character.x_ << "\n";
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

double SimultaneousMazeState::white_score()
{
    double score = characters_[0].game_score_ - characters_[1].game_score_;
    if (score == 0)
        return 0.5;
    if (score > 0)
        return 1.0;
    return 0.0;
}

std::string SimultaneousMazeState::winner()
{
    double score = characters_[0].game_score_ - characters_[1].game_score_;
    if (score == 0)
        return "DRAW";
    if (score > 0)
        return characters_[0].mark_;
    return characters_[1].mark_;
}

void SimultaneousMazeState::print_end_game()
{
    using std::cout;
    using std::endl;
    cout << "score 0: " << characters_[0].game_score_
         << " 1: " << characters_[1].game_score_ << endl;
    cout << "winner: " << winner() << endl;
}
