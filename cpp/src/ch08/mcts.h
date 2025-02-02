#ifndef SRC_CH08_MONTE_CARLO_TREE_SEARCH_H_
#define SRC_CH08_MONTE_CARLO_TREE_SEARCH_H_

#include <memory>
#include "maze_state.h"

constexpr const double C = 1.;             // ucb1 の定数
constexpr const int EXPAND_THRESHOLD = 10; // ノード展開の閾値

class Playout
{
private:
    std::unique_ptr<ConnectFourState> state_;

public:
    Playout(const std::unique_ptr<ConnectFourState> &state)
        : state_(state->clone()){};
    double playout();
};

constexpr const int64_t INF = 100000000LL;

class Node
{
private:
    std::unique_ptr<ConnectFourState> state_;
    double w_;

    // utils
    void _increment(double value);
    double t_() const;

public:
    std::vector<Node> child_nodes_;
    double n_;

    Node(const std::unique_ptr<ConnectFourState> &state)
        : state_(state->clone()), w_(0), n_(0) {}
    double evaluate();
    void expand();
    Node &next_child_node();
    int best_action();

    // utils
    double ucb1(double t) const;
    void print_tree(const int depth = 1) const;
};

int mcts_action(const std::unique_ptr<ConnectFourState> &state,
                const int playout_number,
                const bool should_print = false);

int mcts_action_timebound(const std::unique_ptr<ConnectFourState> &state,
                          const int64_t time_threshold);

#endif
