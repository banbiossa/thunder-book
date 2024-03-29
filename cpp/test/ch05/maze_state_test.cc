#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch05/maze_state.h"

using namespace std;

class MazeStateTest : public ::testing::Test
{
protected:
    AlternateMazeState state;

    MazeStateTest() : state(0) {}
};

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score(A): 0 y:2 x: 1
score(B): 0 y:2 x: 3

493.3
97373
1A6B8
66843
69144
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, IsDone)
{
    EXPECT_FALSE(state.is_done());
}

TEST_F(MazeStateTest, LegalActions)
{
    vector<int> actual = state.legal_actions();
    vector<int> expected = {0, 1, 2, 3};
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, Advance)
{
    state.advance(0);
    EXPECT_EQ(state.characters_[0].mark_, "B");
    EXPECT_EQ(state.characters_[1].y_, 2);
}

TEST_F(MazeStateTest, GetScore)
{
    state.advance(0);
    EXPECT_EQ(state.get_score(), -6);
}

TEST_F(MazeStateTest, TebanScore)
{
    state.advance(0);
    EXPECT_EQ(state.teban_score(), 0);
}

TEST_F(MazeStateTest, WinScore)
{
    state.advance(0);
    EXPECT_EQ(state.win_score(), 1);
}

TEST_F(MazeStateTest, Winner)
{
    state.advance(0);
    EXPECT_EQ(state.winner(), "A");
}

TEST_F(MazeStateTest, GetScoreRate)
{
    state.advance(0);
    state.advance(2);
    EXPECT_EQ(state.get_score_rate(), 0.6);
}

TEST_F(MazeStateTest, PrintEndGame)
{
    state.advance(0);
    state.advance(2);
    state.print_end_game();
}
