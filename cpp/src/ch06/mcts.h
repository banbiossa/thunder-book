#ifndef SRC_CH06_MCTS_H_
#define SRC_CH06_MCTS_H_

#include "maze_state.h"

// make playout a class to avoid
// having to remember to copy
// double playout(State *state);

class Playout
{
private:
    State state_;

public:
    Playout(const State &state) : state_(state){};
    double playout();
};

template <typename T>
class BaseNode
{
private:
    State state_;
    double w_;

    // util
    double ucb1(double t) const;
    void _increment(double value);

public:
    std::vector<T> child_nodes_;
    int n_;

    BaseNode(const State &state) : state_(state), w_(0), n_(0);
    T &next_child_node();

    // override
    virtual void expand();
    virtual double explore();

    // virtual ~BaseNode() = default;
};

class EvenNode : public BaseNode<OddNode>
{
public:
    EvenNode(const State &state) : BaseNode(state){};
    void expand() override;
    double explore() override;
};

class OddNode : public BaseNode<EvenNode>
{
public:
    OddNode(const State &state) : BaseNode(state){};
    void expand() override;
    double explore() override;
};

#endif
