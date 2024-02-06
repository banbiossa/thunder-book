#ifndef SRC_CH06_MCTS_H_
#define SRC_CH06_MCTS_H_

#include "maze_state.h"

constexpr const double C = 1.;
constexpr const int EXPAND_THRESHOLD = 10;

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
protected:
    double w_;

    void _increment(double value);

public:
    State state_;
    std::vector<T> child_nodes_;
    int n_;

    BaseNode(const State &state) : w_(0), state_(state), n_(0){};
    virtual ~BaseNode() = default;
    T &next_child_node();

    // util
    double ucb1(double t) const;

    // override
    virtual void expand();
    virtual double explore();
};

// forward declaration for the class
class EvenNode;
class OddNode;

class EvenNode : public BaseNode<OddNode>
{
public:
    EvenNode(const State &state) : BaseNode(state){};
    void expand() override;
    double explore() override;
};

class OddNode : public BaseNode<EvenNode>
{
private:
    int action0; // action of previous node

public:
    OddNode(const State &state, int action0) : BaseNode(state),
                                               action0(action0){};
    void expand() override;
    double explore() override;
};

#endif
