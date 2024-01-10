#ifndef SRC_CH04_AUTO_MOVE_MAZE_STATE_H_
#define SRC_CH04_AUTO_MOVE_MAZE_STATE_H_

#include <iostream>

constexpr const int H = 5;
constexpr const int W = 5;
constexpr int END_TURN = 5;
constexpr int CHARACTER_N = 3;

struct Coord
{
    /* data */
    int y_;
    int x_;
    Coord(const int y = 0, const int x = 0) : y_(y), x_(x) {}
};

using ScoreType = int64_t;
class AutoMoveMazeState
{
private:
    int points_[H][W] = {};
    int turn_;
    Coord characters_[CHARACTER_N] = {};

public:
    int game_score_;
    ScoreType evaluated_score_;
    AutoMoveMazeState(const int seed);
    bool is_done() const;
    std::string to_string() const;
    void set_character(const int character_id, const int y, const int x);
    ScoreType get_score(bool is_print = false) const;
};

#endif
