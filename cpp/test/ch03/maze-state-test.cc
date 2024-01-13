#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch03/maze_state.h"

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

TEST(MazeState, ToString)
{
    MazeState state = MazeState(0);
    string actual = state.to_string();
    string expected = "turn:\t0\n"
                      "score:\t0\n"
                      R"(3.39
7373
166@
)";
    EXPECT_EQ(actual, expected);
}

TEST(MazeState, Advance)
{
    MazeState state = MazeState(0);
    int point_before = state.game_score_;
    state.advance(1);
    int point_after = state.game_score_;
    EXPECT_EQ(point_after, point_before + 6);
}

TEST(MazeState, IsDone)
{
    MazeState state = MazeState(0);
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

TEST(MazeState, RandomAction)
{
    MazeState state = MazeState(0);
    int actual = random_action(state);
    EXPECT_EQ(actual, 1);
}

TEST(MazeState, PlayGame)
{
    play_game(0);
}
