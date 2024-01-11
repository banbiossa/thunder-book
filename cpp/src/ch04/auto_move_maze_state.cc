#include <sstream>
#include <random>
#include "auto_move_maze_state.h"

AutoMoveMazeState::AutoMoveMazeState(const int seed) : turn_(0),
                                                       game_score_(0),
                                                       evaluated_score_(0)
{
    auto mt_for_construct = std::mt19937(seed);
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            points_[y][x] = mt_for_construct() % 9 + 1;
        }
    }
}

void AutoMoveMazeState::set_character(const int character_id,
                                      const int y,
                                      const int x)
{
    this->characters_[character_id].y_ = y;
    this->characters_[character_id].x_ = x;
}

bool AutoMoveMazeState::is_done() const
{
    return this->turn_ == END_TURN;
}

std::string AutoMoveMazeState::to_string() const
{
    std::stringstream ss;
    ss << "turn:\t" << this->turn_ << "\n";
    ss << "score:\t" << this->game_score_ << "\n";
    auto board_chars = std::vector<std::vector<char>>(H, std::vector<char>(W, '.'));
    for (int h = 0; h < H; h++)
    {
        for (int w = 0; w < W; w++)
        {
            bool is_written = false; // この座標に書く文字が決定したか

            for (const auto &character : this->characters_)
            {
                if (character.y_ == h && character.x_ == w)
                {
                    ss << "@";
                    is_written = true;
                    break;
                }
                board_chars[character.y_][character.x_] = '@';
            }
            if (!is_written)
            {
                if (this->points_[h][w] > 0)
                {
                    ss << points_[h][w];
                }
                else
                {
                    ss << '.';
                }
            }
        }
        ss << '\n';
    }

    return ss.str();
}

ScoreType AutoMoveMazeState::get_score(bool is_print = false) const
{
    auto tmp_state = *this;
    // remove points on character
    for (auto &character : this->characters_)
    {
        auto &point = tmp_state.points_[character.y_][character.x_];
        point = 0;
    }
    // move till game end
    while (!tmp_state.is_done())
    {
        tmp_state.advance();
        if (is_print)
        {
            std::cout << tmp_state.to_string() << std::endl;
        }
    }
    return tmp_state.game_score_;
}

void AutoMoveMazeState::advance()
{
    for (int character_id = 0; character_id < CHARACTER_N; character_id++)
    {
        move_player(character_id);
    }
    for (auto &character : this->characters_)
    {
        auto &point = this->points_[character.y_][character.x_];
        this->game_score_ += point;
        point = 0; // 被ったら消すため
    }
    this->turn_++;
}

void AutoMoveMazeState::move_player(const int character_id)
{
    Coord &character = this->characters_[character_id];
    int best_point = -INF;
    int best_action_index = 0;

    for (int action = 0; action < 4; action++)
    {
        int ty = character.y_ + dy[action];
        int tx = character.x_ + dx[action];
        if (ty >= 0 && ty < H && tx >= 0 && tx < W)
        {
            auto point = this->points_[ty][tx];
            if (point > best_point)
            {
                best_point = point;
                best_action_index = action;
            }
        }
    }
    character.y_ += dy[best_action_index];
    character.x_ += dx[best_action_index];
}

auto mt_for_action = std::mt19937(0);

void AutoMoveMazeState::init()
{
    for (auto &character : characters_)
    {
        character.y_ = mt_for_action() % H;
        character.x_ = mt_for_action() % W;
    }
}

void AutoMoveMazeState::transition()
{
    auto &character = this->characters_[mt_for_action() % CHARACTER_N];
    character.y_ = mt_for_action() % H;
    character.x_ = mt_for_action() % W;
}

void play_game(const StringAIPair &ai, const int seed)
{
    using std::cout;
    using std::endl;
    auto state = State(seed);
    state = ai.second(state);
    cout << state.to_string() << endl;
    auto score = state.get_score(true);
    cout << "Score of " << ai.first << ": " << score << endl;
}
