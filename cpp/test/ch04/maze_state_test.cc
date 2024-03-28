#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch04/auto_move_maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    AutoMoveMazeState state;

    MazeStateTest() : state(0) {}
};

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score: 0
@1378
78221
93742
31164
93939
)";
    EXPECT_EQ(actual, expected);
}
