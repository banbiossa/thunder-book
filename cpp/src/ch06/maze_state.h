#ifndef SRC_CH06_MAZE_STATE_H_
#define SRC_CH06_MAZE_STATE_H_

#include <iostream>

constexpr const int H = 5;
constexpr const int W = 5;
constexpr const int END_TURN = 20;

using ScoreType = int64_t;
constexpr const ScoreType INF = 1000000000LL;

static const std::string dstr[4] = {"RIGHT", "LEFT", "DOWN", "UP"};

struct Character
{
    int y_;
    int x_;
    int game_score_;
    std::string mark_;
    Character(const int y,
              const int x,
              std::string mark) : y_(y),
                                  x_(x),
                                  game_score_(0),
                                  mark_(mark) {}
};

class SimultaneousMazeState
{
private:
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};

    void _advance(const int player_id, const int action);

public:
    std::vector<std::vector<int>> points_;
    int turn_;
    std::vector<Character> characters_;

    SimultaneousMazeState(const int seed);
    bool is_done();
    void advance(const int action0, const int action1);
    std::vector<int> legal_actions(const int player_id) const;
    std::string to_string();
    double white_score();
    std::string winner();
    void print_end_game();
};

using State = SimultaneousMazeState;
using AIFunction = std::function<int(const State &, int)>;

#endif
