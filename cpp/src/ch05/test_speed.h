#ifndef SRC_CH05_TEST_SPEED_H_
#define SRC_CH05_TEST_SPEED_H_

#include <iostream>
#include "maze_state.h"

std::vector<State> get_sample_states(const int game_number);
void calculate_execution_speed(const StringAIPair &ai, const std::vector<State> &states);

#endif
