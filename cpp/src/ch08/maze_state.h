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
protected:
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
    Stone place_stone(const int action);
    void check_connection(const Stone first_stone,
                          const int dx[2],
                          const int dy[2]);

public:
    ConnectFourState() {}
    bool is_done() const;
    virtual std::vector<int> legal_actions() const;
    virtual void advance(const int action);

    // util
    std::string to_string() const;
    double teban_score() const;
    double white_score() const;

}; // ConnectFourState

using AIFunction = std::function<int(const ConnectFourState &state)>;

enum class StateVersion
{
    Normal,
    Bitset,
};

class ConnectFourStateNormal : public ConnectFourState
{
public:
    ConnectFourStateNormal() : ConnectFourState() {}
    std::vector<int> legal_actions() const override;
    void advance(const int action) override;
};

class ConnectFourStateBitset : public ConnectFourState
{
private:
    uint64_t my_bit_board_ = 0ULL;
    u_int64_t all_bit_board_ = 0ULL;

public:
    ConnectFourStateBitset();
    std::vector<int> legal_actions() const override;
    void advance(const int action) override;
    uint64_t get_floor_bit(int w, int h) const;
};

ConnectFourState get_state(StateVersion version);

#endif // SRC_CH08_MAZE_STATE_H_
