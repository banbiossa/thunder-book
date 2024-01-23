#ifndef SRC_CH05_THUNDER_SEARCH_H_
#define SRC_CH05_THUNDER_SEARCH_H_

#include "maze_state.h"

namespace thunder
{
    class Node
    {
    private:
        State state_;
        double w_;

        double _increment(double value);

    public:
        std::vector<Node> child_nodes_;
        int n_;

        Node(const State &state) : state_(state) {}
        double evaluate();
        Node &next_child_node();
        void expand();
        int best_action();
    };

    int thunder_search_action(const State &state,
                              const int playout_number);
}

#endif
