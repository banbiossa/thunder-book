#ifndef SRC_CH06_GAME_H_
#define SRC_CH06_GAME_H_

#include <iostream>
#include "maze_state.h"

float play_game(const int seed, AIFunction actions[2]);
float games_black_and_white(int num_games,
                            AIFunction actions_wb[2],
                            int print_every = 10);

#endif
