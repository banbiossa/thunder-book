#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch06/maze_state.h"
#include "src/ch06/monte_carlo.h"

using namespace std;

class MonteCarloTest : public ::testing::Test
{
protected:
    SimultaneousMazeState state;

    MonteCarloTest() : state(0) {}
};

TEST_F(MonteCarloTest, MonteCarlo)
{
    auto actual = primitive_monte_carlo_action(state, 0, 1000);
    EXPECT_EQ(actual, 0);

    actual = primitive_monte_carlo_action(state, 1, 1000);
    EXPECT_EQ(actual, 1);
}

TEST_F(MonteCarloTest, Playout)
{
    playout(&state);
}
