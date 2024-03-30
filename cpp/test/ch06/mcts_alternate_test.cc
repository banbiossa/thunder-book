#include <iostream>
#include <string>
#include <vector>
#include "gtest/gtest.h"
#include "src/ch06/alternate_maze_state.h"
#include "src/ch06/maze_state.h"
#include "src/ch06/mcts_alternate.h"

using namespace std;
using namespace alternate;

class MCTSTest : public ::testing::Test
{
protected:
    SimultaneousMazeState base_state;
    AlternateMazeState state;
    Node node;

    MCTSTest() : base_state(0), state(base_state, 0), node(state) {}
};

TEST_F(MCTSTest, MCTSAction)
{
    auto actual = mcts_action(base_state, 0, 100);
    EXPECT_LT(actual, 4);
}

TEST_F(MCTSTest, Playout)
{
    auto actual = Playout(state).playout();
    EXPECT_LE(actual, 1);
}

TEST_F(MCTSTest, Expand)
{
    int actual = node.child_nodes_.size();
    EXPECT_EQ(actual, 0);
    node.expand();
    actual = node.child_nodes_.size();
    EXPECT_EQ(actual, 4);
}

TEST_F(MCTSTest, Explore)
{
    auto actual = node.explore();
    EXPECT_LE(actual, 1);
}
