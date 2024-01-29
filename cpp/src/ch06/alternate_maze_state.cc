#include "alternate_maze_state.h"

AlternateMazeState::AlternateMazeState(
    const SimultaneousMazeState &base_state,
    const int player_id) : points_(base_state.points_),
                           turn_(base_state.turn_),
                           characters_(
                               player_id == 0
                                   ? base_state.characters_
                                   : std::vector<Character>{base_state.characters_[1],
                                                            base_state.characters_[0]}){};

bool AlternateMazeState::is_done()
{
    return turn_ == END_TURN_;
}

void AlternateMazeState::advance(const int action)
{
    auto &character = this->characters_[0];
    character.x_ += dx[action];
    character.y_ += dy[action];
    auto &point = this->points_[character.y_][character.x_];
    character.game_score_ += point;
    point = 0;
    this->turn_++;
    std::swap(this->characters_[0], this->characters_[1]);
}

std::vector<int> AlternateMazeState::legal_actions()
{
    std::vector<int> actions;
    constexpr const int player_id = 0;
    const auto &character = this->characters_[player_id];
    for (int action = 0; action < 4; action++)
    {
        int ty = character.y_ + dy[action];
        int tx = character.x_ + dx[action];
        if (ty >= 0 && ty < H && tx >= 0 && tx < W)
        {
            actions.emplace_back(action);
        }
    }
    return actions;
}
