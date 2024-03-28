
#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch03/maze_state.h"
#include "src/ch03/beam_search.h"

using namespace std;

class TimeKeeperTest : public ::testing::Test
{
protected:
    MazeParams params;
    MazeState state;

    TimeKeeperTest() : params{3, 4, 4}, state(0, params) {}
};

TEST_F(TimeKeeperTest, ToString)
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
