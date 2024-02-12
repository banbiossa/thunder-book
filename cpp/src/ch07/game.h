#ifndef SRC_CH07_GAME_H_
#define SRC_CH07_GAME_H_

#include "maze_state.h"

void play_game(AIFunction action_func, const int seed);

enum class StateVersion
{
    BitsetMatrix,
    BitsetSingle,
    Normal,
    Unknown
};

std::shared_ptr<State> get_state(int seed, StateVersion state_version);

double many_games(AIFunction action_func,
                  int num_games,
                  int print_every,
                  StateVersion state_version);

double test_speed(AIFunction action_func,
                  const int game_number,
                  const int per_game_number,
                  int print_every,
                  StateVersion state_version);

#endif
