#ifndef SRC_CH06_MCTS_H_
#define SRC_CH06_MCTS_H_

#include "alternate_maze_state.h"

namespace alternate
{
    using State = AlternateMazeState;

    double playout(State *state);

    constexpr const double C = 1.;
    constexpr const int EXPAND_THRESHOLD = 10;

    class Node
    {
    private:
        State state_;
        double w_;

        // util
        double ucb1(double t) const;

    public:
        Node(State &state);
        std::vector<Node> child_nodes_;
        double n_;

        double explore();
        void expand();
        Node &next_child_node();
    };

}

int mcts_action(const State &base_state,
                const int player_id,
                const int playout_number);

#endif
