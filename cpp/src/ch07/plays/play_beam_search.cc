#include <iostream>
#include "src/ch07/game.h"
#include "src/ch07/beam_search.h"

int main()
{
    using std::cout;
    using std::endl;
    int beam_width = 100;
    int beam_depth = END_TURN;

    cout << "beam search action with"
         << " width " << beam_width
         << " depth " << beam_depth
         << endl;

    AIFunction beam_search_f = [&](const State &state)
    {
        return beam_search_action(state, beam_width, beam_depth);
    };

    double win_rate = many_games(beam_search_f, 100);
    cout << "win rate " << win_rate << endl;
    return 0;
}
