#include <iostream>
#include <string>
#include <vector>
#include "gtest/gtest.h"
#include "src/ch06/alternate_maze_state.h"
#include "src/ch06/maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    SimultaneousMazeState base_state;
    alternate::AlternateMazeState state;

    MazeStateTest() : base_state(0), state(base_state, 0) {}
};

TEST_F(MazeStateTest, LegalActions)
{
    // auto actual = state.legal_actions();
    // auto expected = vector<int>{0, 1, 2, 3};
    // EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, MakeState)
{
    // EXPECT_EQ(state.is_done(), false);
}
