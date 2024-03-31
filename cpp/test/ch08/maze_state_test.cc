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
is_first: 1

.......
.......
.......
.......
.......
.......
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, LegalActions)
{
    vector<int> actual = state.legal_actions();
    vector<int> expected = {0, 1, 2, 3, 4, 5, 6};
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, Advance)
{
    state.advance(0);
    state.advance(1);
    state.advance(2);
    state.advance(3);
    state.advance(4);
    state.advance(5);
    state.advance(6);
    string actual = state.to_string();
    string expected = R"(
)";
    EXPECT_EQ(actual, expected);
}
