#ifndef SRC_CH06_MCTS_H_
#define SRC_CH06_MCTS_H_

#include "maze_state.h"

template <typename T>
class BaseNode
{
public:
    State state;
    virtual ~BaseNode() = default;

    // override
    virtual void expand();
    virtual double explore();
};

#endif
