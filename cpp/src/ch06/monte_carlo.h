#ifndef SRC_CH06_MONTE_CARLO_H_
#define SRC_CH06_MONTE_CARLO_H_

#include "maze_state.h"

double playout(State *state);
int primitive_monte_carlo_action(const State &state,
                                 const int player_id,
                                 const int playout_number);

#endif
