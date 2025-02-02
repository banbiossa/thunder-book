#ifndef SRC_CH05_MAZE_STATE_H_
#define SRC_CH05_MAZE_STATE_H_

#include <iostream>

struct Character
{
    int y_;
    int x_;
    int game_score_;
    std::string mark_;
    Character(const int y = 0,
              const int x = 0,
              std::string mark = "") : y_(y),
                                       x_(x),
                                       game_score_(0),
                                       mark_(mark) {}
};

constexpr const int H = 5;
constexpr const int W = 5;
constexpr const int END_TURN = 10;

using ScoreType = int64_t;
constexpr const ScoreType INF = 100000000LL;

class AlternateMazeState
{
private:
    static constexpr const int dx[4] = {1, -1, 0, 0};
    static constexpr const int dy[4] = {0, 0, 1, -1};

public:
    // members
    std::vector<std::vector<int>> points_;
    int turn_;
    std::vector<Character> characters_;

    // funcs
    AlternateMazeState();
    AlternateMazeState(const int seed);
    bool is_done() const;
    void advance(const int action);
    std::vector<int> legal_actions() const;
    std::string to_string();
    void print_end_game();
    ScoreType get_score() const;
    float win_score();
    float teban_score();
    double get_score_rate() const;

    // util
    std::string winner();
    float winner_to_score(std::string winner);
};

void play_game(const int seed);

using State = AlternateMazeState;
using AIFunction = std::function<int(const State &)>;
using StringAIPair = std::pair<std::string, AIFunction>;

#endif
