#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/iterative_deepening.h"

using namespace std;

class IterativeDeepeningTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    IterativeDeepeningTest() : state(0) {}
};

TEST_F(IterativeDeepeningTest, IterativeDeepeningAction)
{
    int actual = iterative_deepening_action(state, 10);
    int expected = 0;
    EXPECT_EQ(actual, expected);
}
