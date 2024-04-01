#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch08/maze_state.h"
#include "src/ch08/mcts.h"

using namespace std;

class MCTSTest : public ::testing::TestWithParam<StateVersion>
{
protected:
    std::unique_ptr<ConnectFourState> state;
    std::unique_ptr<Node> node;

    void SetUp() override
    {
        state = get_state(GetParam());
        node = std::make_unique<Node>(state->clone());
    }
};

INSTANTIATE_TEST_SUITE_P(
    AllStateVersions,
    MCTSTest,
    ::testing::Values(StateVersion::Normal, StateVersion::Bitset));

TEST_P(MCTSTest, MCTSAction)
{
    int actual = mcts_action(state, 10);
    EXPECT_LE(actual, W);
}

TEST_P(MCTSTest, MCTSActionTimebound)
{
    int actual = mcts_action_timebound(state, 1);
    EXPECT_LE(actual, W);
}

TEST_P(MCTSTest, Expand)
{
    node->expand();
    EXPECT_EQ(node->child_nodes_.size(), state->legal_actions().size());
}

TEST_P(MCTSTest, Playout)
{
    Playout playout = Playout(state->clone());
    double actual = playout.playout();
    EXPECT_LE(actual, 1.0);
}

TEST_P(MCTSTest, Evaluate)
{
    double actual = node->evaluate();
    EXPECT_LE(actual, 1.0);
}

TEST_P(MCTSTest, NextChildNode)
{
    node->expand();
    Node &next_child_node = node->next_child_node();
    EXPECT_LE(next_child_node.evaluate(), 1.0);
}
