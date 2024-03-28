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
                continue;
            if (characters_[1].y_ == y && characters_[1].x_ == x)
                continue;
            this->points_[y][x] = point;
        }
    }
}

bool AlternateMazeState::is_done() const
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

std::string AlternateMazeState::to_string()
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

ScoreType AlternateMazeState::get_score() const
{
    return characters_[0].game_score_ - characters_[1].game_score_;
}

float AlternateMazeState::teban_score()
{
    if (this->get_score() == 0)
        return 0.5;
    if (this->get_score() > 0)
        return 1;
    return 0;
}

float AlternateMazeState::win_score()
{
    std::string winner = this->winner();
    return this->winner_to_score(winner);
}

std::string AlternateMazeState::winner()
{
    auto a = this->characters_[0];
    auto b = this->characters_[1];
    if (a.game_score_ == b.game_score_)
        return "-";
    if (a.game_score_ > b.game_score_)
        return a.mark_;
    else
        return b.mark_;
}

float AlternateMazeState::winner_to_score(std::string winner)
{
    if (winner == "-")
        return 0.5;
    if (winner == "A")
        return 1;
    return 0;
}

double AlternateMazeState::get_score_rate() const
{
    double nominator = (double)characters_[0].game_score_;
    double denominator = (double)(characters_[0].game_score_ +
                                  characters_[1].game_score_);

    // base case, to not divide by 0
    if (denominator == 0)
        return 0.;
    return nominator / denominator;
}
