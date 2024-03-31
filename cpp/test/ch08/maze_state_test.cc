#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch08/maze_state.h"

using namespace std;

class ConnectFourStateTest : public ::testing::TestWithParam<StateVersion>
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
    ConnectFourStateTest,
    ::testing::Values(StateVersion::Normal, StateVersion::Bitset));

TEST_P(ConnectFourStateTest, LegalActions)
{
    std::vector<int> actual = state->legal_actions();
    std::vector<int> expected = {0, 1, 2, 3, 4, 5, 6};
    EXPECT_EQ(actual, expected);
}

class MazeStateTest : public ::testing::Test
{
protected:
    std::unique_ptr<ConnectFourStateNormal> state;

    void SetUp() override
    {
        state = std::make_unique<ConnectFourStateNormal>();
    }
};

class BitsetTest : public ::testing::Test
{
protected:
    std::unique_ptr<ConnectFourStateBitset> state;

    void SetUp() override
    {
        state = std::make_unique<ConnectFourStateBitset>();
    }
};

TEST_P(ConnectFourStateTest, ToString)
{
    string actual = state->to_string();
    string expected = R"(
is_first: 1

.......
.......
.......
.......
.......
.......
)";
    EXPECT_EQ(actual, expected);
}

TEST_P(ConnectFourStateTest, AdvanceOne)
{
    state->advance(1);
    string actual = state->to_string();
    string expected = R"(
is_first: 0

.......
.......
.......
.......
.......
.X.....
)";
    EXPECT_EQ(actual, expected);
}

TEST_P(ConnectFourStateTest, AdvanceMany)
{
    state->advance(0);
    state->advance(1);
    state->advance(2);
    state->advance(3);
    state->advance(4);
    state->advance(5);
    state->advance(6);
    string actual = state->to_string();
    string expected = R"(
is_first: 0

.......
.......
.......
.......
.......
XOXOXOX
)";
    EXPECT_EQ(actual, expected);
}

TEST_P(ConnectFourStateTest, TebanScore)
{
    double actual = state->teban_score();
    double expected = 0.5;
    EXPECT_EQ(actual, expected);

    state->advance(0);
    state->advance(1);
    state->advance(0);
    state->advance(1);
    state->advance(0);
    state->advance(1);

    EXPECT_FALSE(state->is_done());

    state->advance(0);
    actual = state->teban_score();
    expected = 0.0;
    EXPECT_EQ(actual, expected);

    actual = state->white_score();
    expected = 1.0;
    EXPECT_EQ(actual, expected);

    EXPECT_TRUE(state->is_done());
}
