#include "mcts.h"
#include "random_action.h"

double Playout::playout()
{
    if (state_.is_done())
        return state_.white_score();

    state_.advance(random_action(state_, 0),
                   random_action(state_, 1));
    return playout();
}

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
    return w_ / n_ + C * sqrt(2. * log(t) / n_);
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
        t += child_node.n;

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
