#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch08/maze_state.h"
#include "src/ch08/random_action.h"

using namespace std;

class RandomActionTest : public ::testing::TestWithParam<StateVersion>
{
protected:
    std::unique_ptr<ConnectFourState> state;

    void SetUp() override
    {
        state = get_state(GetParam());
    }
};

INSTANTIATE_TEST_SUITE_P(
    AllStateVersions,
    RandomActionTest,
    ::testing::Values(StateVersion::Normal, StateVersion::Bitset));

TEST_P(RandomActionTest, RandomAction)
{
    int actual = random_action(state);
    EXPECT_LE(actual, W);
}
