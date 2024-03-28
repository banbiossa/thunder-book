#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch03/maze_state.h"
#include "src/ch03/beam_search.h"

using namespace std;

class BeamSearchTest : public ::testing::Test
{
protected:
    MazeParams params;
    MazeState state;

    BeamSearchTest() : params{3, 4, 4}, state(0, params) {}
};

TEST_F(BeamSearchTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score: 0
3.39
7373
166@
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(BeamSearchTest, BeamSearchAction)
{
    int actual = beam_search_action(state, 2, params.end_turn_);
    int expected = 1;
    EXPECT_EQ(actual, expected);
}

TEST_F(BeamSearchTest, BeamSearchActionTimed)
{
    int actual = beam_search_action_with_time_threshold(state, 2, 1);
    int expected = 1;
    EXPECT_EQ(actual, expected);
}
