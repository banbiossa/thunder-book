#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch06/maze_state.h"
#include "src/ch06/random_action.h"

using namespace std;

class RandomActionTest : public ::testing::Test
{
protected:
    SimultaneousMazeState state;

    RandomActionTest() : state(0) {}
};

TEST_F(RandomActionTest, RandomAction)
{
    auto actual = random_action(state, 0);
    EXPECT_LT(actual, 4);
}
