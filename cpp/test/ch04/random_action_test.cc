#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch04/auto_move_maze_state.h"
#include "src/ch04/random_action.h"

using namespace std;

class RandomActionTest : public ::testing::Test
{
protected:
    AutoMoveMazeState state;

    RandomActionTest() : state(0) {}
};

TEST_F(RandomActionTest, RandomAction)
{
    EXPECT_EQ(state.characters_[0].x_, 0);
    State actual = random_action(state);
    EXPECT_EQ(actual.characters_[0].x_, 4);
}
