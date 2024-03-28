#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch04/auto_move_maze_state.h"
#include "src/ch04/hill_climb.h"

using namespace std;

class HillClimbTest : public ::testing::Test
{
protected:
    AutoMoveMazeState state;

    HillClimbTest() : state(0) {}
};

TEST_F(HillClimbTest, HillClimb)
{
    EXPECT_EQ(state.characters_[0].x_, 0);
    State actual = hill_climb(state, 100);
    EXPECT_EQ(actual.characters_[0].x_, 4);
    EXPECT_GT(actual.get_score(), state.get_score());
}
