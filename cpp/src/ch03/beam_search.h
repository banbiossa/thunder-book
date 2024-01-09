#ifndef SRC_CH03_BEAM_SEARCH_H
#define SRC_CH03_BEAM_SEARCH_H

#include "maze-state.h"

int beam_search_action(const MazeState &state, const int beam_width, const int beam_depth);
int beam_search_action_with_time_threshold(
    const MazeState &state,
    const int beam_width,
    const int64_t time_threshold);

#endif
