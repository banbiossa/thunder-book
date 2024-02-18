#include <deque>
#include <sstream>
#include "maze_state.h"

bool ConnectFourState::is_done() const
{
    return win_status_ != GameStatus::ONGOING;
}
double ConnectFourState::teban_score() const
{
    switch (win_status_)
    {
    case GameStatus::WIN:
        return 1.0;
    case GameStatus::DRAW:
        return 0.5;
    case GameStatus::LOSE:
        return 0.0;
    default:
        // should not reach any other case
        // if not done, will not call teban_score()
        return 0.5;
    }
}

double ConnectFourState::white_score() const
{
    double score = teban_score();
    if (!is_first_)
        score = 1 - score;
    return score;
}

std::string ConnectFourState::to_string() const
{
    std::stringstream ss("");
    ss << "is_first:\t" << is_first_ << "\n";
    for (int y = H - 1; y >= 0; y--)
    {
        ss << "\n";
        for (int x = 0; x < W; x++)
        {
            ss << get_board_char(y, x);
        } // x
    }     // y

    ss << "\n";
    return ss.str();
}

// start ignore "-Wreturn-type"
// this doesn't return Stone on all paths but we know
// the break condition is always met
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wreturn-type"

Stone ConnectFourStateNormal::place_stone(const int action)
{
    // get first stone to place
    for (int y = 0; y < H; y++)
    {
        if (my_board_[y][action] == 0 && enemy_board_[y][action] == 0)
        {
            my_board_[y][action] = 1;
            return Stone(y, action);
        }
    }
}

#pragma GCC diagnostic pop // end ignore "-Wreturn-type"

void ConnectFourStateNormal::check_connection(const Stone first_stone,
                                              const int dx[2],
                                              const int dy[2])
{
    auto que = std::deque<Stone>();
    que.emplace_back(first_stone);
    std::vector<std::vector<bool>> check(H, std::vector<bool>(W, false));
    int count = 0;

    while (!que.empty())
    {
        const auto &stone = que.front();
        que.pop_front();
        count++;
        if (count >= 4)
        {
            // 自分が揃ったら相手視点は負け
            // is_first の チェックが不要なのかが疑問
            // is_done を踏む瞬間は自分が必ず負けているのでOK
            win_status_ = GameStatus::LOSE;
            return;
        }
        check[stone.y_][stone.x_] = true;

        for (int action = 0; action < 2; action++)
        {
            int ty = stone.y_ + dy[action];
            int tx = stone.x_ + dx[action];

            if (ty >= 0 && ty < H && tx >= 0 && tx < W &&
                my_board_[ty][tx] == 1 && !check[ty][tx])
            {
                que.emplace_back(ty, tx);
            }
        }
    }
}

char ConnectFourStateNormal::get_board_char(int y, int x) const
{
    if (my_board_[y][x] == 1)
        return is_first_ ? 'X' : 'O';
    if (enemy_board_[y][x] == 1)
        return is_first_ ? 'O' : 'X';
    return '.';
}

std::vector<int> ConnectFourStateNormal::legal_actions() const
{
    std::vector<int> actions;
    for (int x = 0; x < W; x++)
    {
        for (int y = 0; y < H; y++)
        {
            if (my_board_[y][x] == 0 && enemy_board_[y][x] == 0)
            {
                actions.emplace_back(x);
                break;
            }
        }
    }
    return actions;
}
void ConnectFourStateNormal::advance(const int action)
{
    Stone stone = place_stone(action);

    // dx の増減をチェックすることで横方向の連携判定
    check_connection(stone, d_up, d_stay);

    if (!is_done())
    {
        // "/" 方向のチェックは {1, -1}, {1, -1}
        check_connection(stone, d_up, d_up);
    }

    if (!is_done())
    {
        // "\" 方向のチェックは {1, -1}, {-1, 1}
        check_connection(stone, d_up, d_down);
    }

    if (!is_done())
    {
        // 上下方向（下方向）のcheck
        // 上には石は無いので若干無駄ではあるが、
        // consistency の方が大事なので
        // {0, 0}, {1, -1} をチェック
        check_connection(stone, d_stay, d_up);
    }

    std::swap(my_board_, enemy_board_);
    is_first_ = !is_first_;
    if (!is_done() && legal_actions().size() == 0)
    {
        win_status_ = GameStatus::DRAW;
    }
}

uint64_t ConnectFourStateBitset::floor_bit(int w, int h) const
{
    // 0b00000010000001...
    // w: 繰り返しの数 h: 0 の数
    uint64_t bit = 0;
    for (int x = 0; x < w; x++)
        bit |= 1ULL << x * (h + 1);
    return bit;
}

uint64_t ConnectFourStateBitset::filled(int w, int h) const
{
    /*
    0b01111110111111...
    makes a head missing bit, this means all  board is filled

    1 << h-1 = 100
    100 - 1 = 011
    011 << x*h = 011000
    の組み合わせ
    */
    uint64_t bit = 0;
    for (int x = 0; x < w; x++)
    {
        bit |= ((1ULL << (h - 1)) - 1) << x * h;
    }
    return bit;
}

std::vector<int> ConnectFourStateBitset::legal_actions() const
{
    std::vector<int> actions;
    uint64_t possible = all_bit_board_ + floor_bit(W, H);
    uint64_t filter = 0b111111;
    for (int x = 0; x < W; x++)
    {
        if ((filter & possible) != 0)
            actions.emplace_back(x);
        filter <<= (H + 1);
    }
    return actions;
}

void ConnectFourStateBitset::advance(const int action)
{
    all_bit_board_ |= (all_bit_board_ + (1ULL << action * (H + 1)));

    if (is_winner(my_bit_board_))
        win_status_ = GameStatus::LOSE;
    else if (all_bit_board_ == filled(W, H))
        win_status_ = GameStatus::DRAW;

    // 敵視点に切り替え
    my_bit_board_ ^= all_bit_board_;
    is_first_ = !is_first_;
}

bool ConnectFourStateBitset::is_winner(const uint64_t board)
{
    // -- dir
    uint64_t tmp_board = board & (board >> 7);
    if ((tmp_board & (tmp_board >> 14)) != 0)
        return true;

    // \ dir
    tmp_board = board & (board >> 6);
    if ((tmp_board & (tmp_board >> 12)) != 0)
        return true;

    // / dir
    tmp_board = board * (board >> 8);
    if ((tmp_board & (tmp_board >> 16)) != 0)
        return true;

    // | dir
    tmp_board = board & (board >> 1);
    if ((tmp_board & (tmp_board >> 2)) != 0)
        return true;

    return false;
}

char ConnectFourStateBitset::get_board_char(int y, int x) const
{
    int index = x * (H + 1) + y;
    uint64_t index_bit = 1ULL << index;

    if ((my_bit_board_ & index_bit) == 1)
        return is_first_ ? 'X' : 'O';

    auto enemy_board = my_bit_board_ ^ all_bit_board_;
    if ((enemy_board & index_bit) == 1)
        return is_first_ ? 'O' : 'X';

    return '.';
}

ConnectFourState get_state(StateVersion version)
{
    switch (version)
    {
    case StateVersion::Normal:
        return ConnectFourStateNormal();
    case StateVersion::Bitset:
        return ConnectFourStateBitset();
    }
}
