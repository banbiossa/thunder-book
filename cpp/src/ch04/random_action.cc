#include <random>
#include "random_action.h"

auto mt_for_action = std::mt19937(0);

State random_action(const State &state)
{
    State now_state = state;
    for (int character_id = 0; character_id < CHARACTER_N; character_id++)
    {
        int y = mt_for_action() % H;
        int x = mt_for_action() % W;

        now_state.set_character(character_id, y, x);
    }
    return now_state;
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

int main()
{
    const auto &ai = StringAIPair(
        "random_action", [&](const State &state)
        { return random_action(state); });
    play_game(ai, 0);
    return 0;
}
