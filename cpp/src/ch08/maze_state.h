#ifndef SRC_CH08_MAZE_STATE_H_
#define SRC_CH08_MAZE_STATE_H_

#include <functional>
#include <vector>

constexpr const int H = 6;
constexpr const int W = 7;

enum class GameStatus
{
    ONGOING,
    WIN,
    LOSE,
    DRAW,
}; // GameStatus

struct Stone
{
    int y_;
    int x_;
    Stone(int y, int x) : y_(y), x_(x) {}
}; // Stone

class ConnectFourState
{
private:
    // consts
    static constexpr const int d_up[2] = {1, -1};
    static constexpr const int d_stay[2] = {0, 0};
    static constexpr const int d_down[2] = {-1, 1};

    // attributues
    bool is_first_ = true;
    int my_board_[H][W] = {};
    int enemy_board_[H][W] = {};
    GameStatus win_status_ = GameStatus::ONGOING;

    // helper functions
    Stone get_first_stone(const int action);
    void check_connection(const Stone first_stone,
                          const int dx[2],
                          const int dy[2]);

public:
    ConnectFourState() {}
    bool is_done() const;
    GameStatus did_i_win() const;
    std::vector<int> legal_actions() const;
    void advance(const int action);
    std::string to_string() const;

}; // ConnectFourState

using AIFunction = std::function<int(const ConnectFourState &state)>;

#endif // SRC_CH08_MAZE_STATE_H_
