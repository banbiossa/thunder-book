#include <deque>
#include <sstream>
#include "maze_state.h"

bool ConnectFourState::is_done() const
{
    return win_status_ != GameStatus::ONGOING;
}

GameStatus ConnectFourState::did_i_win() const
{
    return win_status_;
}

std::vector<int> ConnectFourState::legal_actions() const
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

// start ignore "-Wreturn-type"
// this doesn't return Stone on all paths but we know
// the break condition is always met
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wreturn-type"

Stone ConnectFourState::place_stone(const int action)
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

void ConnectFourState::check_connection(const Stone first_stone,
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

void ConnectFourState::advance(const int action)
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
    if (win_status_ == GameStatus::ONGOING && legal_actions().size() == 0)
    {
        win_status_ = GameStatus::DRAW;
    }
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
            char c = '.';
            if (my_board_[y][x] == 1)
                c = (is_first_ ? 'X' : 'O');
            else if (enemy_board_[y][x] == 1)
                c = (is_first_ ? 'O' : 'X');
            ss << c;
        } // x
    }     // y

    ss << "\n";
    return ss.str();
}

double ConnectFourState::teban_score() const
{
    // base case DRAW
    double score;
    switch (win_status_)
    {
    case GameStatus::WIN:
        /* code */
        score = 1.0;
        break;
    case GameStatus::DRAW:
        score = 0.5;
        break;
    case GameStatus::LOSE:
        score = 0.0;
        break;
    default:
        break;
    }
    // if (!is_first_)
    //     score = 1. - score;
    return score;
}
