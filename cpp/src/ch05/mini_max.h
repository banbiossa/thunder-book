#ifndef SRC_CH05_MINI_MAX_H_
#define SRC_CH05_MINI_MAX_H_

#include "maze_state.h"

using State = AlternateMazeState;

ScoreType mini_max_score(const State &state, const int depth);
int mini_max_action(const State &state, const int depth);

#endif
