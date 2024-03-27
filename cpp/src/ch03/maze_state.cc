#include <iostream>
#include <sstream>
#include <random>
#include "maze_state.h"

MazeState::MazeState(const int seed, const MazeParams &params)
    : params_(params)
{
    auto mt_for_construct = std::mt19937(seed);
    this->character_.y_ = mt_for_construct() % params_.height_;
    this->character_.x_ = mt_for_construct() % params_.width_;

    // init points
    for (int y = 0; y < params_.height_; y++)
        for (int x = 0; x < params_.width_; x++)
        {
            if (character_.on(y, x))
                continue;
            this->points_[y][x] = mt_for_construct() % 10;
        }
}

bool MazeState::is_done() const
{
    return turn_ == params_.end_turn_;
}

void MazeState::advance(const int action)
{
    this->character_.x_ += dx[action];
    this->character_.y_ += dy[action];
    auto &point = this->points_[this->character_.y_][this->character_.x_];
    if (point > 0)
    {
        this->game_score_ += point;
        point = 0;
    }
    this->turn_++;
}

std::vector<int> MazeState::legal_actions() const
{
    std::vector<int> actions;
    for (int action = 0; action < 4; action++)
    {
        int ty = this->character_.y_ + dy[action];
        int tx = this->character_.x_ + dx[action];
        if (ty >= 0 && ty < params_.height_ && tx >= 0 && tx < params_.width_)
        {
            actions.emplace_back(action);
        }
    }
    return actions;
}

std::string MazeState::to_string() const
{
    std::stringstream ss;
    // add \n on the head for easier testing
    ss << "\n";
    ss << "turn: " << this->turn_ << "\n";
    ss << "score: " << this->game_score_ << "\n";
    for (int h = 0; h < params_.height_; h++)
    {
        for (int w = 0; w < params_.width_; w++)
        {
            if (this->character_.y_ == h && this->character_.x_ == w)
                ss << '@';
            else if (this->points_[h][w] > 0)
                ss << points_[h][w];
            else
                ss << '.';
        }
        ss << "\n";
    }
    return ss.str();
}

void MazeState::evaluate_score()
{
    this->evaluated_score_ = this->game_score_;
}

using State = MazeState;

std::mt19937 mt_for_action(0);

int random_action(const State &state)
{
    auto legal_actions = state.legal_actions();
    return legal_actions[mt_for_action() % (legal_actions.size())];
}

void play_game(const int seed, const MazeParams &params)
{
    using std::cout;
    using std::endl;

    auto state = State(seed, params);
    cout << state.to_string() << endl;

    while (!state.is_done())
    {
        state.advance(random_action(state));
        cout << state.to_string() << endl;
    }
}

bool operator<(const MazeState &maze_1, const MazeState &maze_2)
{
    return maze_1.evaluated_score_ < maze_2.evaluated_score_;
}
