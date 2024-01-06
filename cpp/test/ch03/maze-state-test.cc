#include "gtest/gtest.h"
#include "src/ch03/maze-state.h"

TEST(HelloTest, FirstTest)
{
    EXPECT_EQ(1, 1);
}

TEST(MazeState, Init)
{
    MazeState state = MazeState(0);
}
