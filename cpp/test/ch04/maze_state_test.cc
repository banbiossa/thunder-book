#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch04/auto_move_maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    AutoMoveMazeState state;

    MazeStateTest() : state(0) {}
};

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score: 0
@1378
78221
93742
31164
93939
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, IsDone)
{
    EXPECT_FALSE(state.is_done());
}

TEST_F(MazeStateTest, InitCharacter)
{
    state.init();
    EXPECT_EQ(state.characters_[0].y_, 4);
    EXPECT_EQ(state.characters_[0].x_, 4);
    EXPECT_EQ(state.characters_[1].y_, 3);
    EXPECT_EQ(state.characters_[1].x_, 0);
    EXPECT_EQ(state.characters_[2].y_, 3);
    EXPECT_EQ(state.characters_[2].x_, 4);
}

TEST_F(MazeStateTest, Transition)
{
    state.transition();
    EXPECT_EQ(state.characters_[0].y_, 0);
    EXPECT_EQ(state.characters_[0].x_, 0);
    EXPECT_EQ(state.characters_[1].y_, 3);
    EXPECT_EQ(state.characters_[1].x_, 2);
    EXPECT_EQ(state.characters_[2].y_, 0);
    EXPECT_EQ(state.characters_[2].x_, 0);
}

TEST_F(MazeStateTest, SetCharacter)
{
    state.set_character(0, 1, 1);
    EXPECT_EQ(state.characters_[0].y_, 1);
    EXPECT_EQ(state.characters_[0].x_, 1);
}

TEST_F(MazeStateTest, GetScore)
{
    state.init();
    EXPECT_EQ(state.get_score(), 78);
}

TEST_F(MazeStateTest, Advance)
{
    state.init();
    state.advance();
    EXPECT_EQ(state.characters_[0].y_, 1);
    EXPECT_EQ(state.characters_[0].x_, 0);
    EXPECT_EQ(state.characters_[1].y_, 4);
    EXPECT_EQ(state.characters_[1].x_, 4);
    EXPECT_EQ(state.characters_[2].y_, 3);
    EXPECT_EQ(state.characters_[2].x_, 0);
    EXPECT_EQ(state.game_score_, 19);
    EXPECT_EQ(state.turn_, 1);
}

TEST_F(MazeStateTest, MovePlayer)
{
    state.init();
    state.move_player(0);
    EXPECT_EQ(state.characters_[0].y_, 4);
    EXPECT_EQ(state.characters_[0].x_, 2);
}
