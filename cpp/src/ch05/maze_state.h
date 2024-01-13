#ifndef SRC_CH05_MAZE_STATE_H_
#define SRC_CH05_MAZE_STATE_H_

#include <iostream>

struct Character
{
    int y_;
    int x_;
    int game_score_;
    std::string mark_;
    Character(const int y = 0,
              const int x = 0,
              std::string mark = "") : y_(y),
                                       x_(x),
                                       game_score_(0),
                                       mark_(mark) {}
};

enum WinningStatus
{
    WIN,
    LOSE,
    DRAW,
    NONE,
};

constexpr const int H = 3;
constexpr const int W = 3;
constexpr const int END_TURN = 4;

using ScoreType = int64_t;
constexpr const ScoreType INF = 100000000LL;

class AlternateMazeState
{
private:
    std::vector<std::vector<int>> points_;
    int turn_;
    std::vector<Character> characters_;
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};

public:
    AlternateMazeState();
    AlternateMazeState(const int seed);
    bool is_done() const;
    WinningStatus get_winning_status();
    void advance(const int action);
    std::vector<int> legal_actions() const;
    std::string to_string();
    Character get_winner();
    void print_end_game();
    ScoreType get_score() const;
};

void play_game(const int seed);

#endif
