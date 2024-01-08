#ifndef SRC_CH03_BEAM_SEARCH_H
#define SRC_CH03_BEAM_SEARCH_H

#include "maze-state.h"

int beam_search_action(const MazeState &state, const int beam_width, const int beam_depth);

#endif
