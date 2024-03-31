#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch07/maze_state.h"
#include "src/ch07/random_action.h"

using namespace std;

class RandomActionTest : public ::testing::Test
{
protected:
    WallMazeState state;

    RandomActionTest() : state(0) {}
};

TEST_F(RandomActionTest, RandomAction)
{
    int actual = random_action(state);
    EXPECT_LE(actual, 3);
}
