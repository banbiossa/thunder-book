#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch04/auto_move_maze_state.h"

class MazeStateTest : public ::testing::Test
{
protected:
    MazeParams params;
    MazeState state;

    MazeStateTest() : params{3, 4, 4}, state(0, params) {}
};
