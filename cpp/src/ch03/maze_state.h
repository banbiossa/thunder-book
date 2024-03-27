#ifndef SRC_CH03_MAZE_STATE_H_
#define SRC_CH03_MAZE_STATE_H_

#include <iostream>
#include <sstream>
#include <random>

struct Coord
{
    /* data */
    int y_;
    int x_;
    Coord(const int y = 0, const int x = 0) : y_(y), x_(x) {}
    bool on(int y, int x) { return y_ == y && x_ == x; }
};

constexpr const int H = 30;
constexpr const int W = 30;
constexpr int END_TURN = 100;

struct MazeParams
{
    int height_;
    int width_;
    int end_turn_;
};

using ScoreType = int64_t;
constexpr const ScoreType INF = 1000000000LL;

class MazeState
{
private:
    // int points_[H][W] = {};
    int **points_;
    int turn_ = 0;
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};

public:
    const MazeParams params_;
    Coord character_ = Coord();
    int game_score_ = 0;
    ScoreType evaluated_score_ = 0;
    int first_action_ = -1; // root action

    // MazeState() {}
    MazeState(const int seed, const MazeParams &params);
    ~MazeState()
    {
        for (int i = 0; i < params_.height_; ++i)
            delete[] points_[i];
        delete[] points_;
    }
    bool is_done() const;
    void advance(const int action);
    std::vector<int> legal_actions() const;
    std::string to_string() const;
    void evaluate_score();
};

int random_action(const MazeState &state);
void play_game(const int seed);
bool operator<(const MazeState &maze_1, const MazeState &maze_2);

#endif
