#include <iostream>
#include "src/ch07/game.h"
#include "src/ch07/beam_search.h"

int main()
{
    using std::cout;
    using std::endl;
    int beam_width = 2;
    int beam_depth = 100;

    cout << "beam search action with depth "
         << beam_depth << " width " << beam_depth << endl;
    AIFunction beam_search_f = [&](const State &state)
    {
        return beam_search_action(state, beam_width, beam_depth);
    };
    play_game(beam_search_f, 0);
    return 0;
}
