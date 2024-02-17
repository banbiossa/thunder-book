#include <iostream>
#include <assert.h>
#include "mcts.h"
#include "random_action.h"
#include "time_keeper.h"

double Playout::playout()
{
    if (state_.is_done())
        return state_.teban_score();

    state_.advance(random_action(state_));
    return playout();
}

int mcts_action(const ConnectFourState &state,
                const int playout_number,
                const bool should_print)
{
    Node node = Node(state);
    node.expand();
    for (int i = 0; i < playout_number; i++)
    {
        node.evaluate();
        if (should_print)
            node.print_tree();
    }

    return node.best_action();
}

int mcts_action_timebound(const ConnectFourState &state,
                          const int64_t time_threshold)
{
    Node node = Node(state);
    node.expand();
    auto time_keeper = TimeKeeper(time_threshold);
    while (!time_keeper.is_time_over())
        node.evaluate();

    return node.best_action();
}

int Node::best_action()
{
    auto legal_actions = state_.legal_actions();
    int best_action_searched_number = -1;
    int best_action_index = -1;
    assert(legal_actions.size() == child_nodes_.size());

    // 試行回数の多いノードを選ぶ（いいノードは試行回数も多いから)
    for (int i = 0; i < (int)legal_actions.size(); i++)
    {
        int n = child_nodes_[i].n_;
        if (n > best_action_searched_number)
        {
            best_action_index = i;
            best_action_searched_number = n;
        }
    }
    return legal_actions[best_action_index];
}

void Node::_increment(double value)
{
    w_ += value;
    n_++;
}

double Node::evaluate()
{
    if (state_.is_done())
    {
        double value = state_.teban_score();
        _increment(value);
        return value;
    }

    if (child_nodes_.empty())
    {
        double value = Playout(state_).playout();
        _increment(value);
        if (n_ == EXPAND_THRESHOLD)
            expand();
        return value;
    }

    // has child
    double value = 1. - next_child_node().evaluate();
    _increment(value);
    return value;
}

void Node::expand()
{
    auto legal_actions = state_.legal_actions();
    child_nodes_.clear();
    for (const auto action : legal_actions)
    {
        child_nodes_.emplace_back(state_);
        child_nodes_.back().state_.advance(action);
    }
}

double Node::t_() const
{
    double t = 0;
    for (const auto &child_node : child_nodes_)
        t += child_node.n_;
    return t;
}

double Node::ucb1(double t) const
{
    using std::log;
    using std::sqrt;
    return 1 - w_ / n_ + (double)C * sqrt(2. * log(t) / n_);
}

Node &Node::next_child_node()
{
    for (auto &child_node : child_nodes_)
        if (child_node.n_ == 0)
            return child_node;

    // select best ucb1
    double t = t_();
    double best_value = -INF;
    int best_action_index = -1;
    for (int i = 0; i < (int)child_nodes_.size(); i++)
    {
        const auto &child_node = child_nodes_[i];
        double ucb1_value = child_node.ucb1(t);
        if (ucb1_value > best_value)
        {
            best_action_index = i;
            best_value = ucb1_value;
        }
    }
    return child_nodes_[best_action_index];
}

void Node::print_tree(const int depth) const
{
    using std::cout;
    using std::endl;

    for (int i = 0; i < (int)child_nodes_.size(); i++)
    {
        const auto &child_node = child_nodes_[i];
        for (int j = 0; j < depth; j++)
            cout << "__";
        cout << " " << i << "(" << child_node.n_ << ")" << endl;
        if (!child_node.child_nodes_.empty())
        {
            child_node.print_tree(depth + 1);
        }
    }
}
