#ifndef SRC_CH06_MAZE_STATE_H_
#define SRC_CH06_MAZE_STATE_H_

#include <iostream>

constexpr const int H = 3;
constexpr const int W = 3;
constexpr const int END_TURN = 4;

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

class SimultaneousMazeState
{
private:
    std::vector<std::vector<int>> points_;
    int turn_;
    std::vector<Character> characters_;

    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};

    void _advance(const int character, const int action);

public:
    SimultaneousMazeState(const int seed);
    bool is_done();
    void advance(const int action0, const int action1);
    std::vector<int> legal_actions(const int player_id);
    std::string to_string();
};

using State = SimultaneousMazeState;

#endif
