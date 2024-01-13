#ifndef SRC_CH05_RANDOM_ACTION_H_
#define SRC_CH05_RANDOM_ACTION_H_

#include "maze_state.h"

using State = AlternateMazeState;

int random_action(const State &state);
void play_game(const int seed);

#endif
