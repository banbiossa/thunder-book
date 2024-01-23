#include <assert.h>
#include "thunder_search.h"
#include "time_keeper.h"

namespace thunder
{
    double Node::_increment(double value)
    {
        this->w_ += value;
        ++this->n_;
        return value;
    }

    double Node::evaluate()
    {
        if (this->state_.is_done())
        {
            double value = this->state_.teban_score();
            return this->_increment(value);
        }

        if (this->child_nodes_.empty())
        {
            double value = this->state_.get_score_rate();
            this->_increment(value); // expand が入るから return しない
            this->expand();
            return value;
        }

        double value = 1. - this->next_child_node().evaluate();
        return this->_increment(value);
    }

    Node &Node::next_child_node()
    {
        for (auto &child_node : this->child_nodes_)
        {
            if (child_node.n_ == 0)
                return child_node;
        }

        double best_value = -INF;
        int best_action_index = -1;
        for (int i = 0; i < (int)this->child_nodes_.size(); i++)
        {
            const auto &child_node = this->child_nodes_[i];
            double thunder_value = 1. - child_node.w_ / child_node.n_;
            if (thunder_value > best_value)
            {
                best_action_index = i;
                best_value = thunder_value;
            }
        }
        return this->child_nodes_[best_action_index];
    }

    int Node::best_action()
    {
        auto legal_actions = this->state_.legal_actions();
        // return argmax of child_node.n
        int best_action_index = -1;
        int best_score = -1;
        assert(legal_actions.size() == this->child_nodes_.size());
        for (int i = 0; i < (int)legal_actions.size(); i++)
        {
            int n = this->child_nodes_[i].n_;
            if (n > best_score)
            {
                best_action_index = i;
                best_score = n;
            }
        }
        return legal_actions[best_action_index];
    }

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

    int thunder_search_action(const State &state,
                              const int playout_number)
    {
        Node root_node = Node(state);
        root_node.expand();
        for (int i = 0; i < playout_number; i++)
            root_node.evaluate();

        return root_node.best_action();
    }

    int thunder_search_action_with_timekeeper(
        const State &state,
        const int64_t time_threshold)
    {
        Node root_node = Node(state);
        root_node.expand();

        auto time_keeper = TimeKeeper(time_threshold);
        while (!time_keeper.is_time_over())
            root_node.evaluate();

        return root_node.best_action();
    }
}
