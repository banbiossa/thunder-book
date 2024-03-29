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
    int actual = mcts_action_with_time_threshold(state, 10);
    int expected = 3;
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
    EXPECT_EQ(actual, 1);
    EXPECT_EQ(node.n_, 1);
    EXPECT_EQ(node.w_, 1);
}
