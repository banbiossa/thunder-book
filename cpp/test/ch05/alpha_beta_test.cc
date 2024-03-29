#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/alpha_beta.h"

using namespace std;

class AlphaBetaTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    AlphaBetaTest() : state(0) {}
};

TEST_F(AlphaBetaTest, AlphaBetaScore)
{
    ScoreType actual = alpha_beta_score(state, -INF, INF, 3, TimeKeeper(1000));
    ScoreType expected = 8;
    EXPECT_EQ(actual, expected);
}

TEST_F(AlphaBetaTest, AlphaBetaAction)
{
    int actual = alpha_beta_action(state, END_TURN);
    int expected = 0;
    EXPECT_EQ(actual, expected);
}
