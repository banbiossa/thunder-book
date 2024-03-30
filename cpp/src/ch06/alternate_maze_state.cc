#include "alternate_maze_state.h"

namespace alternate
{

    AlternateMazeState::AlternateMazeState(
        const SimultaneousMazeState &base_state,
        const int player_id) : points_(base_state.points_),
                               turn_(base_state.turn_),
                               characters_(base_state.characters_)
    {
        if (player_id == 1)
            std::swap(this->characters_[0], this->characters_[1]);
    }

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

    std::vector<int> AlternateMazeState::legal_actions() const
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

    double AlternateMazeState::white_score()
    {
        auto char_a = characters_[0];
        auto char_b = characters_[1];
        double score = char_a.game_score_ - char_b.game_score_;
        if (score == 0)
            return 0.5;

        if (char_a.mark_ == "B")
            score = -score;

        if (score > 0)
            return 1.0;
        return 0.0;
    }

    double AlternateMazeState::teban_score()
    {
        auto char_a = characters_[0];
        auto char_b = characters_[1];
        double score = char_a.game_score_ - char_b.game_score_;
        if (score == 0)
            return 0.5;
        if (score > 0)
            return 1.0;
        return 0.0;
    }
}
