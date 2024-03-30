#include <assert.h>
#include "mcts_alternate.h"
#include "random_action.h"

namespace alternate
{
    using State = AlternateMazeState;

    double Playout::playout()
    {
        if (state_.is_done())
            return state_.teban_score();

        state_.advance(random_action(state_));
        return 1 - playout();
    }

    Node::Node(State &state) : state_(state),
                               w_(0),
                               n_(0){};

    double Node::explore()
    {
        if (state_.is_done())
        {
            double value = state_.teban_score();
            w_ += value;
            n_ += 1;
            return value;
        }

        if (child_nodes_.empty())
        {
            double value = Playout(state_).playout();
            w_ += value;
            n_++;
            if (n_ == EXPAND_THRESHOLD)
                expand();
            return value;
        }

        // has child
        double value = 1. - next_child_node().explore();
        w_ += value;
        n_++;
        return value;
    };

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

    double Node::ucb1(double t) const
    {
        return 1 - w_ / n_ + (double)C * std::sqrt(2. * std::log(t) / n_);
    }

    Node &Node::next_child_node()
    {
        for (auto &child_node : child_nodes_)
        {
            if (child_node.n_ == 0)
                return child_node;
        }

        double t = 0;
        for (const auto &child_node : child_nodes_)
            t += child_node.n_;

        double best_value = -INF;
        int best_action_index = -1;
        for (int i = 0; i < (int)child_nodes_.size(); i++)
        {
            const auto &child_node = child_nodes_[i];
            double ucb1 = child_node.ucb1(t);
            if (ucb1 > best_value)
            {
                best_action_index = i;
                best_value = ucb1;
            }
        }
        return child_nodes_[best_action_index];
    }

    int mcts_action(const SimultaneousMazeState &base_state,
                    const int player_id,
                    const int playout_number)
    {
        auto state = State(base_state, player_id);
        Node node = alternate::Node(state);
        node.expand();
        for (int i = 0; i < playout_number; i++)
            node.explore();
        auto legal_actions = state.legal_actions();

        int best_action_count = -1;
        int best_index = -1;
        assert(legal_actions.size() == node.child_nodes_.size());

        for (int i = 0; i < (int)legal_actions.size(); i++)
        {
            int n = node.child_nodes_[i].n_;
            if (n > best_action_count)
            {
                best_action_count = n;
                best_index = i;
            }
        }
        return legal_actions[best_index];
    }
}
