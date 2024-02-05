#ifndef SRC_CH06_MCTS_H_
#define SRC_CH06_MCTS_H_

#include "maze_state.h"

template <typename T>
class BaseNode
{
public:
    State state;
    virtual ~BaseNode() = default;

    double ucb1(double t) const;

    // override
    virtual void expand();
    virtual double explore();
};

class EvenNode : public BaseNode<OddNode>
{
public:
    void expand() override;
    double explore() override;
};

class OddNode : public BaseNode<EvenNode>
{
public:
    void expand() override;
    double explore() override;
};

#endif
