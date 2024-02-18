#include <iostream>
#include "src/ch08/random_action.h"
#include "src/ch08/game.h"
#include "src/ch08/mcts.h"

int main()
{
    using std::cout;
    using std::endl;
    int time_threshold = 1;
    int num_games = 100;
    int print_every = 10;

    AIFunction mcts_time_f = [&](const ConnectFourState &state)
    {
        return mcts_action_timebound(state, time_threshold);
    };

    cout << "mcts " << time_threshold << "ms vs_random" << endl;
    AIFunction actions_wb[2] = {mcts_time_f, random_action};

    double win_rate = games_black_and_white(actions_wb,
                                            num_games,
                                            print_every);

    cout << "mcts " << time_threshold << "ms vs_random"
         << " win rate " << win_rate
         << endl;
    return 0;
}
