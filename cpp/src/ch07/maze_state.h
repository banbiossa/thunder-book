#ifndef SRC_Ch07_MAZE_STATE_H_
#define SRC_Ch07_MAZE_STATE_H_

#include <vector>

constexpr const int H = 5;
constexpr const int W = 5;
constexpr const int END_TURN = 4;

struct Character
{
    int y_;
    int x_;
    Character(const int y = 0,
              const int x = 0) : y_(y),
                                 x_(x) {}
};

class WallMazeState
{
private:
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};
    int walls_[H][W] = {};
    int points_[H][W] = {};
    int turn_ = 0;

public:
    Character character_;
    double evaluted_score;
    int first_action = -1;
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
int random_action(const State &state);
void play_game(const int seed);

#endif
