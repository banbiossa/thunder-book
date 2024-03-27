#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch03/maze_state.h"
#include "src/ch03/greedy.h"

using namespace std;

class GreedyTest : public ::testing::Test
{
protected:
    MazeParams params;
    MazeState state;

    GreedyTest() : params{3, 4, 4}, state(0, params) {}
};

TEST_F(GreedyTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score: 0
3.39
7373
166@
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(GreedyTest, GreedyAction)
{
    int actual = greedy_action(state);
    int expected = 1;
    EXPECT_EQ(actual, expected);
}
