#ifndef SRC_CH04_SIMULATED_ANNEALING_H_
#define SRC_CH04_SIMULATED_ANNEALING_H_

#include "auto_move_maze_state.h"

State simulated_annealing(
    const State &state,
    int number,
    double start_temp,
    double end_temp);

#endif
