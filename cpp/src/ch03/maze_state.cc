#include <iostream>
#include <sstream>
#include <random>
#include "maze_state.h"

MazeState::MazeState(const int seed)
{
    auto mt_for_construct = std::mt19937(seed);
    this->character_.y_ = mt_for_construct() % H;
    this->character_.x_ = mt_for_construct() % W;

    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            if (y == character_.y_ && x == character_.x_)
            {
                continue;
            }
            this->points_[y][x] = mt_for_construct() % 10;
        }
    }
}

bool MazeState::is_done() const
{
    return this->turn_ == END_TURN;
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
        if (ty >= 0 && ty < H && tx >= 0 && tx < W)
        {
            actions.emplace_back(action);
        }
    }
    return actions;
}

std::string MazeState::to_string() const
{
    std::stringstream ss;
    ss << "turn:\t" << this->turn_ << "\n";
    ss << "score:\t" << this->game_score_ << "\n";
    for (int h = 0; h < H; h++)
    {
        for (int w = 0; w < W; w++)
        {
            if (this->character_.y_ == h && this->character_.x_ == w)
            {
                ss << '@';
            }
            else if (this->points_[h][w] > 0)
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

void play_game(const int seed)
{
    using std::cout;
    using std::endl;

    auto state = State(seed);
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
