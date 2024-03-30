#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/thunder_search.h"

using namespace std;
using namespace thunder;

class ThunderSearchTest : public ::testing::Test
{
protected:
    AlternateMazeState state;
    Node node;

    ThunderSearchTest() : state(0), node(state) {}
};

TEST_F(ThunderSearchTest, ThunderSearchAction)
{
    int actual = thunder_search_action(state, 10);
    int expected = 0;
    EXPECT_EQ(actual, expected);
}

TEST_F(ThunderSearchTest, ThunderSearchActionWithTime)
{
    int actual = thunder_search_action_with_timekeeper(state, 1);
    int expected = 3;
    EXPECT_EQ(actual, expected);
}

TEST_F(ThunderSearchTest, Expand)
{
    int actual = node.child_nodes_.size();
    int expected = 0;
    EXPECT_EQ(actual, expected);

    node.expand();
    actual = node.child_nodes_.size();
    expected = 4;
    EXPECT_EQ(actual, expected);
}
