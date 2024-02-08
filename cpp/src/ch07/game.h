#ifndef SRC_CH07_GAME_H_
#define SRC_CH07_GAME_H_

#include "maze_state.h"

void play_game(AIFunction action_func, const int seed);
double many_games(AIFunction action_func, int num_games);

#endif
