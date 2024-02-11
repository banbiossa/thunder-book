#include <iostream>
#include "src/ch07/game.h"
#include "src/ch07/beam_search.h"

using std::cout;
using std::endl;

int loop(bool use_bitset)
{
    int beam_width = 100;
    int beam_depth = END_TURN;

    cout << "time beam search action with"
         << " bitset " << use_bitset
         << " width " << beam_width
         << " depth " << beam_depth
         << endl;

    AIFunction beam_search_f = [&](const State &state)
    {
        return beam_search_action(state, beam_width, beam_depth);
    };

    double win_rate = many_games(beam_search_f, 100, 10, use_bitset);
    cout << "win rate " << win_rate << endl;

    cout << endl;
    double speed = test_speed(beam_search_f, 100, 10, 10, use_bitset);
    cout << "average speed " << speed << "ms" << endl;
    return 0;
}

int main()
{
    loop(false);
    cout << endl;
    loop(true);
    return 0;
}
