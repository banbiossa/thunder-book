#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/monte_carlo_tree_search.h"

using namespace std;

class MCTSTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    MCTSTest() : state(0) {}
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
