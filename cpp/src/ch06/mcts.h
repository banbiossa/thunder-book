#ifndef SRC_CH06_MCTS_H_
#define SRC_CH06_MCTS_H_

#include "alternate_maze_state.h"

double playout(AlternateMazeState *state);

constexpr const double C = 1.;
constexpr const int EXPAND_THRESHOLD = 10;

class Node
{
private:
    AlternateMazeState state_;
    double w_;

public:
    std::vector<Node> child_nodes;
    double n_;

    double explore();
};

#endif
