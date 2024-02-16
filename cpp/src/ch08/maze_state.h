#ifndef SRC_CH08_MAZE_STATE_H_
#define SRC_CH08_MAZE_STATE_H_

#include <vector>

constexpr const int H = 6;
constexpr const int W = 7;

enum class GameStatus
{
    WIN,
    LOSE,
    DRAW,
    ONGOING,
}; // GameStatus

class ConnectFourState
{
private:
    bool is_first = true;
    int my_board_[H][W] = {};
    int enemy_board_[H][W] = {};
    GameStatus win_status_;

public:
    ConnectFourState() {}
    bool is_done() const;
    GameStatus did_i_win() const;
    std::vector<int> legal_actions() const;
    void advance(const int action);

}; // ConnectFourState

#endif // SRC_CH08_MAZE_STATE_H_
