#include <iostream>
#include "src/ch07/game.h"
#include "src/ch07/beam_search.h"

using std::cout;
using std::endl;

int loop(StateVersion state_version, std::string version)
{
    int beam_width = 100;
    int beam_depth = END_TURN;
    int num_games = 1;

    cout << "time beam search action with"
         << " version " << version
         << " width " << beam_width
         << " depth " << beam_depth
         << endl;

    AIFunction beam_search_f = [&](const State &state)
    {
        return beam_search_action(state, beam_width, beam_depth, true);
    };

    double win_rate = many_games(beam_search_f, num_games, 10, state_version);
    cout << "win rate " << win_rate << endl;

    cout << endl;
    double speed = test_speed(beam_search_f, num_games, 10, 10, state_version);
    cout << "average speed " << speed << "ms" << endl;
    return 0;
}

int main()
{

    loop(StateVersion::Normal, "normal");
    cout << endl;
    loop(StateVersion::BitsetMatrix, "matrix");
    cout << endl;
    loop(StateVersion::BitsetSingle, "single");
    return 0;
}
