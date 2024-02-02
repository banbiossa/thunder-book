#include <assert.h>
#include "mcts.h"
#include "random_action.h"

namespace alternate
{
    using State = AlternateMazeState;

    double playout(State *state)
    {
        if (state->is_done())
            return state->white_score();
        state->advance(random_action(*state));
        return 1. - playout(state);
    };

    Node::Node(State &state) : state_(state),
                               w_(0),
                               n_(0){};

    double Node::explore()
    {
        if (this->state_.is_done())
        {
            double value = this->state_.white_score();
            this->w_ += value;
            this->n_ += 1;
            return value;
        }

        if (this->child_nodes_.empty())
        {
            State state_copy = this->state_;
            double value = playout(&state_copy);
            this->w_ += value;
            this->n_++;

            if (this->n_ == EXPAND_THRESHOLD)
                this->expand();

            return value;
        }

        // has child
        double value = 1. - this->next_child_node().explore();
        this->w_ += value;
        this->n_++;
        return value;
    };

    void Node::expand()
    {
        auto legal_actions = this->state_.legal_actions();
        this->child_nodes_.clear();
        for (const auto action : legal_actions)
        {
            this->child_nodes_.emplace_back(this->state_);
            this->child_nodes_.back().state_.advance(action);
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
