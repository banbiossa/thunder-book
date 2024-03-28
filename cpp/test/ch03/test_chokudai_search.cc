#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch03/maze_state.h"
#include "src/ch03/chokudai_search.h"

using namespace std;

class ChokudaiTest : public ::testing::Test
{
protected:
    MazeParams params;
    MazeState state;

    ChokudaiTest() : params{3, 4, 4}, state(0, params) {}
};

TEST_F(ChokudaiTest, ToString)
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
