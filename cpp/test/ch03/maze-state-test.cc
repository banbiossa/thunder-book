#include "gtest/gtest.h"
#include "src/ch03/maze-state.h"

using namespace std;

TEST(HelloTest, FirstTest)
{
    EXPECT_EQ(1, 1);
}

TEST(MazeState, Init)
{
    MazeState state = MazeState(0);
    // if seed is working
    EXPECT_EQ(state.character_.x_, 3);
    EXPECT_EQ(state.character_.y_, 2);

    // sum of points is not 0
}

TEST(MazeState, LegalActions)
{
    MazeState state = MazeState(0);
    auto actual = state.legal_actions();
    std::vector<int> expected = {1, 3};
    EXPECT_EQ(actual, expected);
}
