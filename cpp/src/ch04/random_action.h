#ifndef SRC_CH04_RANDOM_ACTION_H_
#define SRC_CH04_RANDOM_ACTION_H_

#include "auto_move_maze_state.h"

State random_action(const State &state);
void play_game(const StringAIPair &ai, const int seed);

#endif
