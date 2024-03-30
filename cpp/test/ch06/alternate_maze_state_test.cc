#include <iostream>
#include <string>
#include <vector>
#include "gtest/gtest.h"
#include "src/ch06/alternate_maze_state.h"
#include "src/ch06/maze_state.h"

using namespace std;

class AlternateMazeStateTest : public ::testing::Test
{
protected:
    SimultaneousMazeState base_state;
    alternate::AlternateMazeState state;

    AlternateMazeStateTest() : base_state(0), state(base_state, 0) {}
};

TEST_F(AlternateMazeStateTest, LegalActions)
{
    auto actual = state.legal_actions();
    auto expected = vector<int>{0, 1, 2, 3};
    EXPECT_EQ(actual, expected);
}

TEST_F(AlternateMazeStateTest, MakeState)
{
    EXPECT_EQ(state.is_done(), false);
}

TEST_F(AlternateMazeStateTest, Advance)
{
    state.advance(0);
    auto actual = state.legal_actions();
    auto expected = vector<int>{0, 1, 2, 3};
    EXPECT_EQ(actual, expected);
}
