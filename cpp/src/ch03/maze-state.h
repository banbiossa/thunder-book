#ifndef SRC_CH03_MAZE_STATE_H_
#define SRC_CH03_MAZE_STATE_H_

struct Coord
{
    /* data */
    int y_;
    int x_;
    Coord(const int y = 0, const int x = 0) : y_(y), x_(x) {}
};


constexpr const int H = 3;
constexpr const int W = 4;
constexpr int END_TURN = 4;

class MazeState
{
private:
    int points_[H][W] = {};
    int turn_ = 0;
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};

public:
    Coord character_ = Coord();
    int game_score_ = 0;
    MazeState() {}
    MazeState(const int seed);

};

void play_game(const int seed);

#endif
