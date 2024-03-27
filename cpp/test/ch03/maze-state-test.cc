#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch03/maze_state.h"

using namespace std;

TEST(HelloTest, FirstTest)
{
    EXPECT_EQ(1, 1);
}

class MazeStateTest : public ::testing::Test
{
protected:
    MazeParams params;
    MazeState state;

    MazeStateTest() : params{3, 4, 4}, state(0, params) {}
};

TEST_F(MazeStateTest, Init)
{
    // if seed is working
    EXPECT_EQ(state.character_.x_, 3);
    EXPECT_EQ(state.character_.y_, 2);

    // sum of points is not 0
}

TEST_F(MazeStateTest, LegalActions)
{
    auto actual = state.legal_actions();
    std::vector<int> expected = {1, 3};
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, ToString)
{
    string actual = state.to_string();
    string expected = R"(
turn: 0
score: 0
3.39
7373
166@
)";
    EXPECT_EQ(actual, expected);
}

TEST_F(MazeStateTest, Advance)
{
    int point_before = state.game_score_;
    state.advance(1);
    int point_after = state.game_score_;
    EXPECT_EQ(point_after, point_before + 6);
}

TEST_F(MazeStateTest, IsDone)
{
    EXPECT_EQ(state.is_done(), false);
    state.advance(1);
    EXPECT_EQ(state.is_done(), false);
    state.advance(0);
    EXPECT_EQ(state.is_done(), false);
    state.advance(1);
    EXPECT_EQ(state.is_done(), false);
    state.advance(0);
    EXPECT_EQ(state.is_done(), true);
}

TEST_F(MazeStateTest, RandomAction)
{
    int actual = random_action(state);
    EXPECT_EQ(actual, 1);
}

TEST_F(MazeStateTest, PlayGame)
{
    play_game(0, params);
}

TEST(MazeState, CopyConstructor)
{
    // Create a MazeParams object with sample dimensions
    MazeParams params{5, 5, 10};

    // Create a MazeState object
    MazeState state(0, params);

    // Fill the points_ array with some sample values
    for (int i = 0; i < params.height_; ++i)
    {
        for (int j = 0; j < params.width_; ++j)
        {
            state.points_[i][j] = i * params.width_ + j;
        }
    }

    // Create a copy of the MazeState object using the copy constructor
    MazeState copy(state);

    // Check that the points_ array is deeply copied
    EXPECT_NE(state.points_, copy.points_);

    // Check that the values in the copied points_ array match the original
    for (int i = 0; i < params.height_; ++i)
    {
        EXPECT_NE(state.points_[i], copy.points_[i]);
        for (int j = 0; j < params.width_; ++j)
        {
            EXPECT_EQ(state.points_[i][j], copy.points_[i][j]);
        }
    }

    // Check that other member variables are correctly copied
    EXPECT_EQ(state.params_.height_, copy.params_.height_);
    EXPECT_EQ(state.params_.width_, copy.params_.width_);
    EXPECT_EQ(state.params_.end_turn_, copy.params_.end_turn_);
    EXPECT_EQ(state.turn_, copy.turn_);
    EXPECT_EQ(state.character_.y_, copy.character_.y_);
    EXPECT_EQ(state.character_.x_, copy.character_.x_);
    EXPECT_EQ(state.game_score_, copy.game_score_);
}
