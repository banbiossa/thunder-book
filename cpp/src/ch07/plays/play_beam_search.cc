#include <functional>
#include <iostream>
#include "src/ch07/game.h"
#include "src/ch07/beam_search.h"

using std::cout;
using std::endl;

void loop(bool use_zobrist_hash)
{
    int beam_width = 100;
    int beam_depth = END_TURN;

    cout << "beam search action with"
         << " width " << beam_width
         << " depth " << beam_depth
         << " no zobrist hash "
         << endl;

    AIFunction beam_search_f_no_hash = [&](const State &state)
    {
        return beam_search_action(state, beam_width, beam_depth, use_zobrist_hash);
    };

    double win_rate = many_games(beam_search_f_no_hash, 100);
    cout << "win rate " << win_rate << endl;
}

int main()
{
    loop(false);
    cout << endl;
    loop(true);

    return 0;
}
