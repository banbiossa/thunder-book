#ifndef SRC_CH06_RANDOM_ACTION_H_
#define SRC_CH06_RANDOM_ACTION_H_

#include "maze_state.h"
#include "alternate_maze_state.h"

int random_action(const State &state, const int player_id);

namespace alternate
{
    int random_action(const AlternateMazeState &state);
}

#endif
