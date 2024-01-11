#ifndef SRC_CH04_HILL_CLIMB_H
#define SRC_CH04_HILL_CLIMB_H

#include "auto_move_maze_state.h"

using State = AutoMoveMazeState;

State hill_climb(const State &state, int number);
int main();

#endif
