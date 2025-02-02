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
    // attributues
    bool is_first_ = true;
    GameStatus win_status_ = GameStatus::ONGOING;

public:
    bool is_done() const;
    double teban_score() const;
    double white_score() const;

    virtual ~ConnectFourState() = default;
    // to override
    virtual std::vector<int> legal_actions() const = 0;
    virtual void advance(const int action) = 0;
    virtual std::unique_ptr<ConnectFourState> clone() const = 0;
    virtual std::string to_string() const = 0;

}; // ConnectFourState

using AIFunction = std::function<int(const std::unique_ptr<ConnectFourState> &state)>;

class ConnectFourStateNormal : public ConnectFourState
{
private:
    // consts
    static constexpr const int d_up[2] = {1, -1};
    static constexpr const int d_stay[2] = {0, 0};
    static constexpr const int d_down[2] = {-1, 1};

    int my_board_[H][W] = {};
    int enemy_board_[H][W] = {};

    // helper functions
    Stone place_stone(const int action);
    void check_connection(const Stone first_stone,
                          const int dx[2],
                          const int dy[2]);

public:
    ConnectFourStateNormal() {}
    virtual std::string to_string() const override;
    std::vector<int> legal_actions() const override;
    void advance(const int action) override;
    std::unique_ptr<ConnectFourState> clone() const override
    {
        return std::make_unique<ConnectFourStateNormal>(*this);
    }
};

class ConnectFourStateBitset : public ConnectFourState
{
private:
    uint64_t my_bit_board_ = 0ULL;
    uint64_t all_bit_board_ = 0ULL;

    // helpers
    uint64_t floor_bit(int w, int h) const;
    uint64_t filled(int w, int h) const;
    bool is_winner(const uint64_t board);

public:
    ConnectFourStateBitset(){};
    virtual std::string to_string() const override;
    std::vector<int> legal_actions() const override;
    void advance(const int action) override;
    std::unique_ptr<ConnectFourState> clone() const override
    {
        return std::make_unique<ConnectFourStateBitset>(*this);
    }
};

enum class StateVersion
{
    Normal,
    Bitset,
};
std::unique_ptr<ConnectFourState> get_state(StateVersion version);

#endif // SRC_CH08_MAZE_STATE_H_
