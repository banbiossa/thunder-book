#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/monte_carlo.h"

using namespace std;

class MonteCarloTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    MonteCarloTest() : state(0) {}
};

TEST_F(MonteCarloTest, Playout)
{
    ScoreType actual = playout(&state);
    EXPECT_EQ(actual, 1);
}

TEST_F(MonteCarloTest, MonteCarloAction)
{
    int actual = primitive_monte_carlo_action(state, 3000);
    int expected = 2;
    EXPECT_EQ(actual, expected);
}
