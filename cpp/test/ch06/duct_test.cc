#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch06/maze_state.h"
#include "src/ch06/duct.h"

using namespace std;
using namespace duct;

class DuctTest : public ::testing::Test
{
protected:
    SimultaneousMazeState state;
    Node node;

    DuctTest() : state(0), node(state) {}
};

TEST_F(DuctTest, Duct)
{
    auto actual = duct_action(state, 0, 3000);
    EXPECT_EQ(actual, 0);

    actual = duct_action(state, 1, 3000);
    EXPECT_EQ(actual, 1);
}

TEST_F(DuctTest, Expand)
{
    int actual = node.child_nodeses_.size();
    EXPECT_EQ(actual, 0);
    node.expand();
    actual = node.child_nodeses_.size();
    EXPECT_EQ(actual, 4);
}

TEST_F(DuctTest, Explore)
{
    double actual = node.explore();
    EXPECT_GE(actual, 0);
}

TEST_F(DuctTest, NextChildNode)
{
    node.expand();
    auto &actual = node.next_child_node();
    EXPECT_EQ(actual.n_, 0);
}
TEST_F(DuctTest, Action0)
{
    for (int i = 0; i < 1000; i++)
        node.explore();
    int actual = node.action0();
    EXPECT_GE(actual, 0);
}
