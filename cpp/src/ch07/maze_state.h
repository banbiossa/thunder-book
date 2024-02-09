#ifndef SRC_Ch07_MAZE_STATE_H_
#define SRC_Ch07_MAZE_STATE_H_

#include <functional>
#include <vector>

constexpr const int H = 7;
constexpr const int W = 7;
constexpr const int END_TURN = 49;

struct Character
{
    int y_;
    int x_;
    Character(const int y = 0,
              const int x = 0) : y_(y),
                                 x_(x) {}
};

struct DistanceCoord
{
    int y_;
    int x_;
    int distance_;
    DistanceCoord() : y_(0), x_(0), distance_(0) {}
    DistanceCoord(const int y,
                  const int x,
                  const int distance) : y_(y),
                                        x_(x),
                                        distance_(distance) {}
    DistanceCoord(const Character &c) : y_(c.y_),
                                        x_(c.x_),
                                        distance_(0) {}
};

class WallMazeState
{
private:
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};
    int walls_[H][W] = {};
    int points_[H][W] = {};

public:
    int turn_ = 0;
    Character character_;
    int first_action_ = -1;
    int game_score_ = 0;
    double evaluated_score_ = 0;

    WallMazeState(const int seed);
    std::vector<int> legal_actions() const;
    bool is_done() const;
    void evaluate_score();
    void advance(const int action);
    std::string to_string();
};

using State = WallMazeState;

bool operator<(const State &maze_1, const State &maze_2);

using AIFunction = std::function<int(const State &)>;

#endif
