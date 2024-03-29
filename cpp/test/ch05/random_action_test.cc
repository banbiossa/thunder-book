#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"
#include "src/ch05/random_action.h"

using namespace std;

class RandomActionTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    RandomActionTest() : state(0) {}
};

TEST_F(RandomActionTest, RandomActionScore)
{
    auto actual = random_action(state);
    auto legal_actions = state.legal_actions();
    bool contains = std::find(legal_actions.begin(), legal_actions.end(), actual) != legal_actions.end();
    EXPECT_TRUE(contains);
}
