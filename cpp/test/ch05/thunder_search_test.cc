#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/thunder_search.h"

using namespace std;
using namespace thunder;

class ThunderSearchTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    ThunderSearchTest() : state(0) {}
};

TEST_F(ThunderSearchTest, ThunderSearchAction)
{
    int actual = thunder_search_action(state, 10);
    int expected = 0;
    EXPECT_EQ(actual, expected);
}
