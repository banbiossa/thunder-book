#ifndef SRC_CH05_WIN_RATE_H_
#define SRC_CH05_WIN_RATE_H_

#include "maze_state.h"

float one_game(const int seed, AIFunction actions[2]);

float white_games(int num_games,
                  AIFunction actions_wb[2],
                  int print_every);

float games_black_and_white(int num_games,
                            AIFunction actions_bw[2],
                            int print_every = 10);

#endif
