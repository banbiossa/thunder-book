#ifndef SRC_CH08_MONTE_CARLO_TREE_SEARCH_H_
#define SRC_CH08_MONTE_CARLO_TREE_SEARCH_H_

#include "maze_state.h"

constexpr const double C = 1.;             // ucb1 の定数
constexpr const int EXPAND_THRESHOLD = 10; // ノード展開の閾値

class Playout
{
private:
    ConnectFourState state_;

public:
    Playout(const ConnectFourState &state) : state_(state){};
    double playout();
};

constexpr const int64_t INF = 100000000LL;

class Node
{
private:
    ConnectFourState state_;
    double w_;

    // utils
    void _increment(double value);
    double t_() const;

public:
    std::vector<Node> child_nodes_;
    double n_;

    Node(const ConnectFourState &state) : state_(state), w_(0), n_(0) {}
    double evaluate();
    void expand();
    Node &next_child_node();
    void print_tree(const int depth = 1) const;
    int best_action();

    // utils
    double ucb1(double t) const;
};

int mcts_action(const ConnectFourState &state,
                const int playout_number,
                const bool should_print = false);

#endif
