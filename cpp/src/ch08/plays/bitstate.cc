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

    cout << "mcts " << time_threshold << "ms bitstate vs normal" << endl;
    AIFunction actions_wb[2] = {mcts_time_f, random_action};
    StateVersion versions[2] = {StateVersion::Bitset, StateVersion::Bitset};

    // double win_rate = games_black_and_white(actions_wb,
    //                                         num_games,
    //                                         print_every);
    double win_rate = many_games_with_state(actions_wb,
                                            versions,
                                            num_games,
                                            print_every);

    cout
        << "mcts " << time_threshold << "ms bitstate vs normal"
        << " win rate " << win_rate
        << endl;
    return 0;
}
