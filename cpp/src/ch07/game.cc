#include <random>
#include <iostream>
#include "game.h"
#include "bitset_matrix.h"
#include "bitset_single.h"
#include "globals.h"

using std::cout;
using std::endl;

void play_game(AIFunction action_func, const int seed)
{

    std::string action_to_str[4] = {"RIGHT", "LEFT", "DOWN", "UP"};

    auto state = State(seed);
    cout << state.to_string() << endl;

    while (!state.is_done())
    {
        cout << "turn " << state.turn_ << endl;

        int action = action_func(state);
        cout << "action " << action << " "
             << action_to_str[action] << endl;
        state.advance(action);
        cout << state.to_string() << endl;
    }
}

State get_state(int seed, StateVersion state_version)
{
    switch (state_version)
    {
    case StateVersion::BitsetMatrix:
        cout << "return BitsetState" << endl;
        return BitsetState(seed);
    case StateVersion::BitsetSingle:
        cout << "return SingleBittsetState" << endl;
        return SingleBitsetState(seed);

    case StateVersion::Normal:
    case StateVersion::Unknown:
    default:
        cout << "return State" << endl;
        return State(seed);
    }
}

double many_games(AIFunction action_func,
                  int num_games,
                  int print_every,
                  StateVersion state_version)
{
    call(__func__);
    double total = 0;
    for (int i = 0; i < num_games; i++)
    {
        auto state = get_state(i, state_version);
        while (!state.is_done())
        {
            state.advance(action_func(state));
        }
        total += state.game_score_;
        if (print_every > 0 && (i % print_every) == 0)
        {
            std::cout << "i " << i << " w "
                      << total / (i + 1) << std::endl;
        }
    }
    return total / (double)num_games;
}

double test_speed(AIFunction action_func,
                  const int game_number,
                  const int per_game_number,
                  int print_every,
                  StateVersion state_version)
{
    using std::cout;
    using std::endl;
    using std::chrono::duration_cast;
    using std::chrono::milliseconds;
    std::chrono::high_resolution_clock::time_point diff_sum;

    for (int i = 0; i < game_number; i++)
    {
        std::mt19937 mt_for_construct(0);
        int seed = mt_for_construct();
        auto state = get_state(seed, state_version);

        auto start_time = std::chrono::high_resolution_clock::now();
        for (int j = 0; j < per_game_number; j++)
            action_func(state);
        auto diff = std::chrono::high_resolution_clock::now() - start_time;
        diff_sum += diff;
        if (print_every > 0 && (i % print_every) == 0)
        {
            double time_mean = duration_cast<milliseconds>(diff_sum.time_since_epoch()).count() / (double)(i + 1);
            std::cout << "i " << i << " time " << time_mean << std::endl;
        }
    }

    double time_mean = duration_cast<milliseconds>(diff_sum.time_since_epoch()).count() / (double)game_number;
    return time_mean;
}
