#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch07/maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    WallMazeState state;

    MazeStateTest() : state(0) {}
};

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score: 0

66986#6
##8###4
3691441
9###@#9
9.123.5
##5###2
91883#6
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, IsDone)
{
    EXPECT_EQ(state.is_done(), false);
}

TEST_F(MazeStateTest, LegalActions)
{
    vector<int> actual = state.legal_actions();
    vector<int> expected = {2, 3};
    EXPECT_EQ(actual, expected);
}
