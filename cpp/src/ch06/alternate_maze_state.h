#ifndef SRC_CH06_ALTERNATE_MAZE_STATE_H_
#define SRC_CH06_ALTERNATE_MAZE_STATE_H_

#include "maze_state.h"

class AlternateMazeState
{
private:
    static constexpr const int END_TURN_ = END_TURN * 2;
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};
    std::vector<std::vector<int>> points_;
    int turn_;
    std::vector<Character> characters_;

public:
    AlternateMazeState(const SimultaneousMazeState &base_state,
                       const int player_id);
    bool is_done();
    void advance(const int action);
    std::vector<int> legal_actions();
};

#endif
