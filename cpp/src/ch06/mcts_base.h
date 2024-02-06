#ifndef SRC_CH06_MCTS_BASE_H_
#define SRC_CH06_MCTS_BASE_H_

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
    virtual void expand() = 0;
    virtual double explore() = 0;
};

template <typename T>
void BaseNode<T>::_increment(double value)
{
    w_ += value;
    n_++;
}

template <typename T>
double BaseNode<T>::ucb1(double t) const
{
    using std::log;
    using std::sqrt;
    // 1. because we want the child's value
    return 1. - w_ / n_ + C * sqrt(2. * log(t) / n_);
};

template <typename T>
T &BaseNode<T>::next_child_node()
{
    // 0 を優先的に
    for (auto &child_node : child_nodes_)
        if (child_node.n_ == 0)
            return child_node;

    // get argmax of ucb1
    double best_value = -INF;
    int best_index = -1;

    int t = 0;
    for (auto &child_node : child_nodes_)
        t += child_node.n_;

    for (int i = 0; i < (int)child_nodes_.size(); i++)
    {
        const auto &child_node = child_nodes_[i];
        double ucb = child_node.ucb1(t);
        if (ucb > best_value)
        {
            best_value = ucb;
            best_index = i;
        }
    }
    return child_nodes_[best_index];
}

#endif
