#ifndef SRC_CH04_AUTO_MOVE_MAZE_STATE_H_
#define SRC_CH04_AUTO_MOVE_MAZE_STATE_H_

#include <iostream>
#include <random>

auto mt_for_action = std::mt19937(0);

constexpr const int H = 5;
constexpr const int W = 5;
constexpr int END_TURN = 5;
constexpr int CHARACTER_N = 3;

using ScoreType = int64_t;
constexpr const ScoreType INF = 1000000000LL;

struct Coord
{
    /* data */
    int y_;
    int x_;
    Coord(const int y = 0, const int x = 0) : y_(y), x_(x) {}
};

class AutoMoveMazeState
{
private:
    int points_[H][W] = {};
    int turn_;
    Coord characters_[CHARACTER_N] = {};
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};

public:
    int game_score_;
    ScoreType evaluated_score_;
    AutoMoveMazeState(const int seed);
    bool is_done() const;
    std::string to_string() const;
    void set_character(const int character_id, const int y, const int x);
    ScoreType get_score(bool is_print = false) const;
    void advance();
    void move_player(const int character_id);
    void init();
    void transition();
};

using State = AutoMoveMazeState;
using AIFunction = std::function<State(const State &)>;
using StringAIPair = std::pair<std::string, AIFunction>;
void play_game(const StringAIPair &ai, const int seed);

#endif
