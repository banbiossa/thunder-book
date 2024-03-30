#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch06/maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    SimultaneousMazeState state;

    MazeStateTest() : state(0) {}
};

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score(A): 0 y:2 x: 1
score(B): 0 y:2 x: 3

3.3.3
37373
8A6B8
34843
44144
)";
    EXPECT_EQ(actual, expected);
}
