#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch07/maze_state.h"
#include "src/ch07/beam_search.h"

using namespace std;

class BeamSearchTest : public ::testing::Test
{
protected:
    WallMazeState state;

    BeamSearchTest() : state(0) {}
};

TEST_F(BeamSearchTest, BeamSearch)
{
    int actual = beam_search_action(state, 10, END_TURN, true);
    EXPECT_EQ(actual, 3);

    actual = beam_search_action(state, 10, END_TURN, false);
    EXPECT_EQ(actual, 3);
}
