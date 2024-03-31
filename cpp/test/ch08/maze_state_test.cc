#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch08/maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    ConnectFourStateNormal state;

    MazeStateTest() : state() {}
};

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
)";
    EXPECT_EQ(actual, expected);
}
