#ifndef SRC_CH05_WIN_RATE_H_
#define SRC_CH05_WIN_RATE_H_

#include "maze_state.h"

float one_game(const int seed, AIFunction actions[2]);
float games_black_and_white(int num_games, AIFunction actions_bw[2]);

#endif
