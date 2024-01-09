
#ifndef SRC_CH03_CHOKUDAI_SEARCH_H_
#define SRC_CH03_CHOKUDAI_SEARCH_H_

#include "maze-state.h"

int chokudai_search_action(
    const MazeState &state,
    const int beam_width,
    const int beam_depth,
    const int beam_number);

int chokudai_search_action_with_time_threshold(
    const MazeState &state,
    const int beam_width,
    const int beam_depth,
    const int64_t time_threshold);

#endif
