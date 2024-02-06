#include <assert.h>
#include "mcts_node.h"
#include "random_action.h"

void EvenNode::expand()
{
    auto legal_actions = state_.legal_actions(0);
    child_nodes_.clear();
    for (const auto &action : legal_actions)
        child_nodes_.emplace_back(OddNode(state_, action));
}

double EvenNode::explore()
{
    if (child_nodes_.empty())
        expand();
    double value = next_child_node().explore();
    _increment(value);
    return value;
}

int EvenNode::best_action(int player_id) const
{
    // for player0
    // for player1 is a combination of these (sums of sums)
    int most_searched_count = 0;
    int best_index = -1;
    auto legal_actions = state_.legal_actions(player_id);
    for (int i = 0; i < (int)legal_actions.size(); i++)
    {
        auto &child_node = child_nodes_[i];
        int searched_count = child_node.n_;
        if (searched_count > most_searched_count)
        {
            most_searched_count = searched_count;
            best_index = i;
        }
    }
    return legal_actions[best_index];
}

void OddNode::expand()
{
    auto legal_actions = state_.legal_actions(1);
    child_nodes_.clear();
    for (const auto &action1 : legal_actions)
    {
        child_nodes_.emplace_back(EvenNode(state_));
        child_nodes_.back().state_.advance(action0, action1);
    }
}

double OddNode::explore()
{
    if (state_.is_done())
    {
        double value = state_.white_score();
        _increment(1. - value);
        return value;
    }

    if (!child_nodes_.empty())
    {
        double value = next_child_node().explore();
        _increment(1. - value);
        return value;
    }

    // no child nodes, return playout
    double value = Playout(state_).playout();
    if (n_ >= EXPAND_THRESHOLD)
        expand();
    _increment(1. - value);
    return value;
}

int mcts_action(State state, int player_id, int playout_number)
{
    assert(player_id == 0);
    EvenNode node = EvenNode(state);
    for (int i = 0; i < playout_number; i++)
        node.explore();

    // get best action, argmax of n
    return node.best_action(player_id);
}
