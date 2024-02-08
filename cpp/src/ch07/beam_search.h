#ifndef SRC_CH07_BEAM_SEARCH_H_
#define SRC_CH07_BEAM_SEARCH_H_

#include "maze_state.h"

int beam_search_action(const State &state,
                       const int beam_width,
                       const int beam_depth);

#endif
