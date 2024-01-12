#include <random>
#include "simulated_annealing.h"

auto mt_for_simulated = std::mt19937(0);

State simulated_annealing(
    const State &state,
    int number,
    double start_temp,
    double end_temp)
{
    State now_state = state;
    now_state.init();
    ScoreType best_score = now_state.get_score();
    ScoreType now_score = best_score;
    State best_state = now_state;

    for (int i = 0; i < number; i++)
    {
        State next_state = now_state;
        next_state.transition();
        auto next_score = next_state.get_score();
        double temp = start_temp + (end_temp - start_temp) * (i / number);
        double probability = exp((next_score - now_score) / temp);
        bool is_force_next = probability > (mt_for_simulated() % INF) / (double)INF;
        if (next_score > now_score || is_force_next)
        {
            now_score = next_score;
            now_state = next_state;
        }
        if (next_score > best_score)
        {
            best_score = next_score;
            best_state = next_state;
        }
    }
    return best_state;
}
