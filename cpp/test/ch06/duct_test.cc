#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch06/maze_state.h"
#include "src/ch06/duct.h"

using namespace std;
using namespace duct;

class DuctTest : public ::testing::Test
{
protected:
    SimultaneousMazeState state;

    DuctTest() : state(0) {}
};

TEST_F(DuctTest, Duct)
{
    auto actual = duct_action(state, 0, 3000);
    EXPECT_EQ(actual, 0);

    actual = duct_action(state, 1, 3000);
    EXPECT_EQ(actual, 1);
}
