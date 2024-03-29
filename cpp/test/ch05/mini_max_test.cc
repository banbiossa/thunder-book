#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/mini_max.h"

using namespace std;

class MiniMaxTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    MiniMaxTest() : state(0) {}
};

TEST_F(MiniMaxTest, MiniMaxScore)
{
    ScoreType actual = mini_max_score(state, 3);
    ScoreType expected = 8;
    EXPECT_EQ(actual, expected);
}

TEST_F(MiniMaxTest, MiniMaxAction)
{
    int actual = mini_max_action(state, 3);
    int expected = 2;
    EXPECT_EQ(actual, expected);
}
