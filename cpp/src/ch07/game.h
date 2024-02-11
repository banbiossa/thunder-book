#ifndef SRC_CH07_GAME_H_
#define SRC_CH07_GAME_H_

#include "maze_state.h"

void play_game(AIFunction action_func, const int seed);

double many_games(AIFunction action_func,
                  int num_games,
                  int print_every,
                  bool use_bitset);

double test_speed(AIFunction action_func,
                  const int game_number,
                  const int per_game_number,
                  int print_every,
                  bool use_bitset);

#endif
