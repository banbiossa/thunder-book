#ifndef SRC_CH05_MONTE_CARLO_TREE_SEARCH_H_
#define SRC_CH05_MONTE_CARLO_TREE_SEARCH_H_

#include "maze_state.h"

constexpr const double C = 1.;             // ucb1 の定数
constexpr const int EXPAND_THRESHOLD = 10; // ノード展開の閾値

class Node
{
private:
    State state_;
    double w_;

public:
    std::vector<Node> child_nodes_;
    double n_;

    Node(const State &state) : state_(state), w_(0), n_(0) {}
    double evaluate();
    void expand();
    Node &next_child_node();
    void print_tree(const int depth = 1) const;
    int best_action();
};

int mcts_action(const State &state,
                const int playout_number,
                const bool should_print = false);

int mcts_action_with_time_threshold(
    const State &state,
    const int64_t time_threshold);

#endif
