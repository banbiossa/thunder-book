#include <random>
#include "auto_move_maze_state.h"
#include "hill_climb.h"
#include "random_action.h"
#include "simulated_annealing.h"

void test_ai_score(const StringAIPair &ai, const int game_number)
{
    using std::cout;
    using std::endl;
    std::mt19937 mt_for_constructor(0);
    double score_mean = 0;
    // time it
    auto start = std::chrono::system_clock::now();
    for (int i = 0; i < game_number; i++)
    {
        auto state = State(mt_for_constructor());
        state = ai.second(state);
        auto score = state.get_score(false);
        score_mean += score;
    }
    auto elpased_in_ms = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now() - start).count();
    score_mean /= (double)game_number;
    cout << ai.first << ":\t"
         << score_mean << " elpased: " << elpased_in_ms << "ms" << endl;
}

int main()
{
    int simulate_number = 10000;
    const std::vector<StringAIPair> ais =
        {
            StringAIPair("random", random_action),
            StringAIPair("hill_climb", [&](const State &state)
                         { return hill_climb(state, simulate_number); }),
            StringAIPair("simulated_annealing", [&](const State &state)
                         { return simulated_annealing(
                               state,
                               simulate_number,
                               /* start_temp */ 500,
                               /* end_temp */ 10); })

        };
    int game_number = 100;
    // print what we will do
    using std::cout;
    using std::endl;
    cout << "play " << game_number << " games with " << simulate_number << " simulations" << endl;
    for (const auto &ai : ais)
    {
        cout << ai.first << " ";
    }
    cout << "\n\n";

    // the actual simulation
    for (const auto &ai : ais)
    {
        test_ai_score(ai, game_number);
    }
    return 0;
}
