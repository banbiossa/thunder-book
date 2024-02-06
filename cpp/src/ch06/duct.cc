#include "monte_carlo.h"
#include "duct.h"

namespace duct
{

    Node::Node(const State &state) : state_(state),
                                     w_(0),
                                     n_(0) {}

    void Node::expand()
    {
        auto legal_actions0 = state_.legal_actions(0);
        auto legal_actions1 = state_.legal_actions(1);
        child_nodeses_.clear();
        for (const auto &action0 : legal_actions0)
        {
            child_nodeses_.emplace_back();
            auto &target_nodes = child_nodeses_.back();
            for (const auto &action1 : legal_actions1)
            {
                target_nodes.emplace_back(state_);
                auto &target_node = target_nodes.back();
                target_node.state_.advance(action0, action1);
            }
        }
    };

    double Node::explore()
    {

        if (state_.is_done())
        {
            double value = state_.white_score();
            w_ += value;
            n_++;
            return value;
        }

        if (child_nodeses_.empty())
        {
            State state_copy = state_;
            double value = playout(&state_copy);
            w_ += value;
            n_++;
            if (n_ == EXPAND_THRESHOLD)
                expand();
            return value;
        }

        // base has child nodeses
        double value = next_child_node().explore();
        w_ += value;
        n_++;
        return value;
    };

    double Node::t() const
    {
        // 分母の計算
        double t = 0;
        for (auto &child_nodes : child_nodeses_)
            for (auto &child_node : child_nodes)
                t += child_node.n_;
        return t;
    }

    double Node::ucb1(double w, double n) const
    {
        using std::log;
        using std::sqrt;
        return w / n + (double)C * sqrt(2. * log(t()) / n);
    }

    int Node::action0() const
    {
        double best_value = -INF;
        double best_i = -1;
        for (int i = 0; i < (int)child_nodeses_.size(); i++)
        {
            const auto &child_nodes = child_nodeses_[i];
            double w = 0;
            double n = 0;
            for (int j = 0; j < (int)child_nodes.size(); j++)
            {
                const auto &child_node = child_nodes[j];
                w += child_node.w_;
                n += child_node.n_;
            }
            double ucb1_value = ucb1(w, n);
            if (ucb1_value > best_value)
            {
                best_i = i;
                best_value = ucb1_value;
            }
        }
        return best_i;
    }

    int Node::action1() const
    {
        double best_value = -INF;
        double best_j = -1;
        for (int j = 0; j < (int)child_nodeses_[0].size(); j++)
        {
            double w = 0;
            double n = 0;
            for (int i = 0; i < (int)child_nodeses_.size(); i++)
            {
                const auto &child_node = child_nodeses_[i][j];
                w += child_node.w_;
                n += child_node.n_;
            }
            w = 1. - w;
            double ucb1_value = ucb1(w, n);
            if (ucb1_value > best_value)
            {
                best_j = j;
                best_value = ucb1_value;
            }
        }
        return best_j;
    }

    Node &Node::next_child_node()
    {
        // 0 のものは優先的に
        for (auto &child_nodes : child_nodeses_)
            for (auto &child_node : child_nodes)
                if (child_node.n_ == 0)
                    return child_node;

        // 0, 1 それぞれの最善を選ぶ
        int best_i = action0();
        int best_j = action1();

        return child_nodeses_[best_i][best_j];
    };

    int duct_action(const State &state,
                    const int player_id,
                    const int playout_number)
    {
        Node node = Node(state);
        node.expand();
        for (int i = 0; i < playout_number; i++)
            node.explore();

        auto legal_actions = state.legal_actions(player_id);
        int i_size = node.child_nodeses_.size();
        int j_size = node.child_nodeses_[0].size();
        // うまく 0,1 の切り替えできる方法ありそうだけど思いつかない

        if (player_id == 0)
        {
            int best_action_count = -1;
            int best_i = -1;
            for (int i = 0; i < i_size; i++)
            {
                int n = 0;
                for (int j = 0; j < j_size; j++)
                    n += node.child_nodeses_[i][j].n_;
                if (n > best_action_count)
                {
                    best_action_count = n;
                    best_i = i;
                }
            }
            return legal_actions[best_i];
        }
        else
        {
            int best_action_count = -1;
            int best_j = -1;
            for (int j = 0; j < j_size; j++)
            {
                int n = 0;
                for (int i = 0; i < i_size; i++)
                    n += node.child_nodeses_[i][j].n_;
                if (n > best_action_count)
                {
                    best_action_count = n;
                    best_j = j;
                }
            }
            return legal_actions[best_j];
        }
    }

}
