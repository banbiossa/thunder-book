#include <random>
#include "test_speed.h"
#include "random_action.h"

std::vector<State> get_sample_states(const int game_number)
{
    std::mt19937 mt_for_sample_states(0);
    std::vector<State> states;
    for (int i = 0; i < game_number; i++)
    {
        auto state = State(mt_for_sample_states());
        int turn = mt_for_sample_states() % END_TURN;
        for (int t = 0; t < turn; t++)
        {
            state.advance(random_action(state));
        }
        states.emplace_back(state);
    }
    return states;
}

void calculate_execution_speed(const StringAIPair &ai,
                               const std::vector<State> &states)
{
    using std::cout;
    using std::endl;
    auto start_time = std::chrono::high_resolution_clock::now();
    for (const auto &state : states)
    {
        ai.second(state);
    }
    auto diff = std::chrono::high_resolution_clock::now() - start_time;
    auto time = std::chrono::duration_cast<std::chrono::milliseconds>(diff).count();
    cout << ai.first << "\ttook " << time << "\tms to process "
         << states.size() << " nodes " << endl;
}
