#ifndef SRC_CH05_ALPHA_BETA_H_
#define SRC_CH05_ALPHA_BETA_H_

#include "maze_state.h"
#include "time_keeper.h"

ScoreType alpha_beta_score(
    const State &state,
    ScoreType alpha,
    const ScoreType beta,
    const int depth,
    const TimeKeeper &time_keeper = TimeKeeper(INT64_MAX));

int alpha_beta_action(
    const State &state,
    const int depth,
    const TimeKeeper &time_keeper = TimeKeeper(/* large default */ INT64_MAX));

#endif
