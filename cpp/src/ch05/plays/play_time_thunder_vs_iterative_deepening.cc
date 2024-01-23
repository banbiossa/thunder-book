#include "src/ch05/win_rate.h"
#include "src/ch05/thunder_search.h"
#include "src/ch05/iterative_deepening.h"

int main()
{
    using std::cout;
    using std::endl;

    int num_games = 100;
    int64_t time_threshold = 1;

    AIFunction thunder_search_timebound = [&](const State &state)
    {
        return thunder::thunder_search_action_with_timekeeper(
            state, time_threshold);
    };
    AIFunction iterative_deepening_timebound = [&](const State &state)
    { return iterative_deepening_action(state, time_threshold); };

    AIFunction actions_wb[2] = {thunder_search_timebound,
                                iterative_deepening_timebound};

    float win_rate = games_black_and_white(num_games, actions_wb);

    cout << "win rate of thunder vs iterative deepening under "
         << time_threshold << "ms is " << win_rate
         << " over " << num_games << " games." << endl;

    return 0;
}
