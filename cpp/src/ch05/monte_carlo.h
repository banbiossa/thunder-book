#ifndef SRC_CH05_MONTE_CARLO_H_
#define SRC_CH05_MONTE_CARLO_H_

#include "maze_state.h"

double playout(State *state);
int primitive_monte_carlo_action(const State &state, int playout_number);

#endif
