
#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch04/auto_move_maze_state.h"
#include "src/ch04/simulated_annealing.h"

using namespace std;

class SimulatedAnnealingTest : public ::testing::Test
{
protected:
    AutoMoveMazeState state;

    SimulatedAnnealingTest() : state(0) {}
};

TEST_F(SimulatedAnnealingTest, SimulatedAnnealing)
{
    EXPECT_EQ(state.characters_[0].x_, 0);
    State actual = simulated_annealing(state, 100, 100, 0);
    EXPECT_EQ(actual.characters_[0].x_, 2);
    EXPECT_GT(actual.get_score(), state.get_score());
}
