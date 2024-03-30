#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/monte_carlo_tree_search.h"

using namespace std;

class MCTSTest : public ::testing::Test
{
protected:
    AlternateMazeState state;
    Node node;

    MCTSTest() : state(0), node(state) {}
};

TEST_F(MCTSTest, MCTSAction)
{
    int actual = mcts_action(state, 3000);
    int expected = 2;
    EXPECT_EQ(actual, expected);
}

TEST_F(MCTSTest, MCTSActionTimebound)
{
    int actual = mcts_action_with_time_threshold(state, 100);
    int expected = 0;
    EXPECT_EQ(actual, expected);
}

TEST_F(MCTSTest, Expand)
{
    int actual = node.child_nodes_.size();
    int expected = 0;
    EXPECT_EQ(actual, expected);

    node.expand();
    actual = node.child_nodes_.size();
    expected = 4;
    EXPECT_EQ(actual, expected);
}

TEST_F(MCTSTest, Evaluate)
{
    double actual = node.evaluate();
    EXPECT_LE(actual, 1);
    EXPECT_EQ(node.n_, 1);
    EXPECT_EQ(node.w_, 1);
}

TEST_F(MCTSTest, NextChildNode)
{
    node.expand();
    Node actual = node.next_child_node();
    Node expected = node.child_nodes_[0];
    EXPECT_EQ(actual.n_, expected.n_);
}

TEST_F(MCTSTest, BestAction)
{
    for (int i = 0; i < 20; i++)
        node.evaluate();
    int actual = node.best_action();
    EXPECT_LE(actual, 4);
}
