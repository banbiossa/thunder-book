#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    MazeStateTest() : state(0) {}
};

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score(A): 0 y:2 x: 1
score(B): 0 y:2 x: 3

493.3
97373
1A6B8
66843
69144
)";
    EXPECT_EQ(actual, expected);
}
