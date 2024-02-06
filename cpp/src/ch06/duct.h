#ifndef SRC_CH06_DUCT_H_
#define SRC_CH06_DUCT_H_

#include "maze_state.h"

namespace duct
{

    constexpr const double C = 1.;
    constexpr const int EXPAND_THRESHOLD = 5;

    class Node
    {
    private:
        State state_;
        double w_;

        double t() const;
        double ucb1(double w, double n) const;

    public:
        std::vector<std::vector<Node>> child_nodeses_;
        int n_;

        int action0() const;
        int action1() const;

        Node(const State &state);
        double explore();
        void expand();
        Node &next_child_node();
    };

    int duct_action(const State &state,
                    const int player_id,
                    const int playout_number);
}

#endif
