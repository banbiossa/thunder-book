#include <assert.h>
#include "monte_carlo.h"
#include "monte_carlo_tree_search.h"

int mcts_action(const State &state, const int playout_number, const bool should_print)
{
    Node root_node = Node(state);
    root_node.expand();
    for (int i = 0; i < playout_number; i++)
    {
        root_node.evaluate();
    }
    auto legal_actions = state.legal_actions();
    int best_action_searched_number = -1;
    int best_action_index = -1;
    assert(legal_actions.size() == root_node.child_nodes_.size());

    // 試行回数の多いノードを選ぶ（いいノードは試行回数も多いから)
    for (int i = 0; i < legal_actions.size(); i++)
    {
        int n = root_node.child_nodes_[i].n_;
        if (n > best_action_searched_number)
        {
            best_action_index = i;
            best_action_searched_number = n;
        }
    }
    {
        static bool called = false;
        if (should_print && !called)
        {
            std::cout << __func__ << std::endl;
            root_node.print_tree();
        }
        called = true;
    }
    return legal_actions[best_action_index];
}

double Node::evaluate()
{
    if (this->state_.is_done())
    {
        double value = this->state_.teban_score();
        this->w_ += value;
        ++this->n_;
        return value;
    }

    if (this->child_nodes_.empty())
    {
        State state_copy = this->state_;
        double value = playout(&state_copy);
        this->w_ += value;
        ++this->n_;

        if (this->n_ == EXPAND_THRESHOLD)
            this->expand();

        return value;
    }

    // has child
    double value = 1. - this->next_child_node().evaluate();
    this->w_ += value;
    ++this->n_;
    return value;
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

Node &Node::next_child_node()
{
    for (auto &child_node : this->child_nodes_)
    {
        if (child_node.n_ == 0)
            return child_node;
    }

    double t = 0;
    for (const auto &child_node : this->child_nodes_)
        t += child_node.n_;
    double best_value = -INF;
    int best_action_index = -1;
    for (int i = 0; i < this->child_nodes_.size(); i++)
    {
        const auto &child_node = this->child_nodes_[i];
        double ucb1_value =
            1. - child_node.w_ / child_node.n_ +
            (double)C * std::sqrt(2. * std::log(t) / child_node.n_);
        if (ucb1_value > best_value)
        {
            best_action_index = i;
            best_value = ucb1_value;
        }
    }
    return this->child_nodes_[best_action_index];
}

void Node::print_tree(const int depth) const
{
    using std::cout;
    using std::endl;

    for (int i = 0; i < child_nodes_.size(); i++)
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
