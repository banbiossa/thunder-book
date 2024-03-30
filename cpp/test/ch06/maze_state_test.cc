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

TEST_F(MazeStateTest, IsDone)
{
    EXPECT_EQ(state.is_done(), false);
}

TEST_F(MazeStateTest, Advance)
{
    state.advance(0, 0);
    string actual = state.to_string();
    string expected = R"(
turn: 1
score(A): 6 y:2 x: 2
score(B): 8 y:2 x: 4

3.3.3
37373
8.A.B
34843
44144
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, LegalActions)
{
    vector<int> actual = state.legal_actions(0);
    vector<int> expected = {0, 1, 2, 3};
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, WhiteScore)
{
    state.advance(0, 0);
    double actual = state.white_score();
    double expected = 0;
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, Winner)
{
    state.advance(0, 0);
    string actual = state.winner();
    string expected = "B";
    EXPECT_EQ(actual, expected);
}
