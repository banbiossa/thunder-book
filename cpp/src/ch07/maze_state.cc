#include <sstream>
#include <random>
#include <iostream>
#include <deque>
#include <assert.h>
#include "maze_state.h"

State::State(const int seed)
{
    auto mt_for_construct = std::mt19937(seed);
    character_.x_ = mt_for_construct() % H;
    character_.y_ = mt_for_construct() % W;

    // 棒倒し法
    for (int y = 1; y < H; y += 2)
    {
        for (int x = 1; x < W; x += 2)
        {
            // (ty, tx) は1マス置き
            if (y == character_.y_ && x == character_.x_)
                continue;
            walls_[y][x] = 1;

            // 最初だけ上も, 壁は(右下左)
            int direction_size = (y == 1) ? 4 : 3;

            int direction = mt_for_construct() % direction_size;
            int ty = y + dy[direction];
            int tx = x + dx[direction];

            // 隣接
            if (ty == character_.y_ && tx == character_.x_)
                continue;
            walls_[ty][tx] = 1;
        }
    }

    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            if (y == character_.y_ && x == character_.x_)
                continue;
            if (walls_[y][x] == 1)
                continue;
            points_[y][x] = mt_for_construct() % 10;
        }
    }

    // add hash
    init_hash();
}

std::vector<int> State::legal_actions() const
{
    std::vector<int> actions;
    for (int action = 0; action < 4; action++)
    {
        int ty = character_.y_ + dy[action];
        int tx = character_.x_ + dx[action];
        if (ty >= 0 && ty < H && tx >= 0 && tx < W && walls_[ty][tx] == 0)
        {
            actions.emplace_back(action);
        }
    }
    return actions;
}

bool State::is_done() const
{
    return turn_ >= END_TURN;
}

void State::evaluate_score()
{
    evaluated_score_ = game_score_ * H * W - get_distance_to_nearest_point();
}

void State::advance(const int action)
{
    // delete character hash (adding xor will delete)
    hash_ ^= zobrist_.z_character_[character_.y_][character_.x_];
    character_.y_ += dy[action];
    character_.x_ += dx[action];
    // add next point character hash
    hash_ ^= zobrist_.z_character_[character_.y_][character_.x_];

    int point = points_[character_.y_][character_.x_];
    if (point > 0)
    {
        assert(point < 10);
        hash_ ^= zobrist_.z_points_[character_.y_][character_.y_][point];
        game_score_ += point;
        points_[character_.y_][character_.x_] = 0;
    }
    turn_++;
}

std::string State::to_string()
{
    std::stringstream ss;
    ss << "turn:\t" << turn_ << "\n";
    ss << "score:\t" << game_score_ << "\n";
    for (int y = 0; y < H; y++)
    {
        ss << "\n";
        for (int x = 0; x < W; x++)
        {
            if (walls_[y][x] == 1)
                ss << "#";
            else if (character_.y_ == y && character_.x_ == x)
                ss << "@";
            else if (points_[y][x] > 0)
                ss << points_[y][x];
            else
                ss << ".";
        }
    }
    ss << "\n";
    return ss.str();
}

bool operator<(const State &maze_1, const State &maze_2)
{
    return maze_1.evaluated_score_ < maze_2.evaluated_score_;
}

ZobristHash::ZobristHash()
{
    std::mt19937 mt_init_hash(0);
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            for (int p = 1; p < 9 + 1; p++)
            {
                z_points_[y][x][p] = mt_init_hash();
            }
            z_character_[y][x] = mt_init_hash();
        }
    }
}

void State::init_hash()
{
    zobrist_ = ZobristHash();
    hash_ = 0;
    hash_ ^= zobrist_.z_character_[character_.y_][character_.x_];
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            auto point = points_[y][x];
            if (point > 0)
                hash_ ^= zobrist_.z_points_[y][x][point];
        }
    }
}

int WallMazeState::get_distance_to_nearest_point()
{
    auto que = std::deque<DistanceCoord>();
    que.emplace_back(character_);
    std::vector<std::vector<bool>> check(H, std::vector<bool>(W, false));
    while (!que.empty())
    {
        const auto &coord = que.front();
        que.pop_front();
        if (points_[coord.y_][coord.x_] > 0)
            return coord.distance_;
        check[coord.y_][coord.x_] = true;

        for (int action = 0; action < 4; action++)
        {
            int ty = coord.y_ + dy[action];
            int tx = coord.x_ + dx[action];

            if (ty >= 0 && ty < H && tx >= 0 && tx < W &&
                !walls_[ty][tx] && !check[ty][tx])
            {
                que.emplace_back(ty, tx, coord.distance_ + 1);
            }
        }
    }
    // 見つからない場合は、maze length の最大値
    return H * W;
}

std::shared_ptr<State> WallMazeState::clone() const
{
    return std::make_shared<WallMazeState>(*this);
}

void State::ref_init()
{
    ref_count_ = 1;
}

void State::ref_add()
{
    ref_count_++;
}

void State::ref_release()
{
    if (ref_count_ == 1)
        delete (this);
    else
        ref_count_--;
}
