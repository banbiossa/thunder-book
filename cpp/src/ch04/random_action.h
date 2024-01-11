#ifndef SRC_CH04_RANDOM_ACTION_H_
#define SRC_CH04_RANDOM_ACTION_H_

#include "auto_move_maze_state.h"

using State = AutoMoveMazeState;
using AIFunction = std::function<State(const State &)>;
using StringAIPair = std::pair<std::string, AIFunction>;

State random_action(const State &state);
void play_game(const StringAIPair &ai, const int seed);

#endif
