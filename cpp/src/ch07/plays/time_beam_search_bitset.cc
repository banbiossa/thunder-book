#include <iostream>
#include "src/ch07/game.h"
#include "src/ch07/beam_search.h"
#include "src/util.h"

using std::cout;
using std::endl;

void loop(StateVersion state_version, std::string version)
{
    int beam_width = 100;
    int beam_depth = END_TURN;
    int num_games = 100;

    cout << "time beam search action with"
         << " version " << version
         << " width " << beam_width
         << " depth " << beam_depth
         << endl;

    AIFunction beam_search_f = [&](const State &state)
    {
        return beam_search_action(state, beam_width, beam_depth, true);
    };

    double score = many_games(beam_search_f, num_games, 10, state_version);
    cout << "score " << score << endl;

    cout << endl;
    double speed = test_speed(beam_search_f, num_games, 10, 10, state_version);
    cout << "average speed " << speed << "ms" << endl;

    // print results
    log_to_file("| %s | %.2f | %.2f ms |", version.c_str(), score, speed);
}

int main()
{
    log_to_file("| name | score | speed |");
    log_to_file("| ---- | ----- | ----- |");
    loop(StateVersion::Normal, "normal");
    cout << endl;
    loop(StateVersion::BitsetMatrix, "matrix");
    cout << endl;
    loop(StateVersion::BitsetSingle, "single");
    return 0;
}
