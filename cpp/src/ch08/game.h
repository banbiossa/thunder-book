#ifndef SRC_CH08_GAME_H_
#define SRC_CH08_GAME_H_

#include "maze_state.h"

double play_game(AIFunction actions_wb[2], bool should_print);

double many_games(AIFunction actions_wb[2],
                  int num_games,
                  int print_every);

double games_black_and_white(AIFunction actions_wb[2],
                             int num_games,
                             int print_every);

double play_game_with_state(AIFunction actions_wb[2],
                            StateVersion state_versions[2]);

double many_games_with_state(AIFunction actions_wb[2],
                             StateVersion state_versions[2],
                             int num_games,
                             int print_every);

#endif // SRC_CH08_GAME_H_
