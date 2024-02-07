
#include "src/ch06/game.h"
#include "src/ch06/mcts_node.h"
#include "src/ch06/mcts_alternate.h"

using std::cout;
using std::endl;

int main()
{
    int playout_number = 30;
    AIFunction mcts_f = [&](const State &state, int player_id)
    {
        return mcts_action(state, player_id, playout_number);
    };
    AIFunction mcts_alternate_f = [&](const State &state, int player_id)
    {
        return alternate::mcts_action(state, player_id, playout_number);
    };

    AIFunction actions_wb[2] = {mcts_f, mcts_alternate_f};

    // play black and white
    int num_games = 100;
    float win_rate = white_games(num_games, actions_wb, /* print every */ 10);

    cout << "mcts vs mcts_alternate " << playout_number << " over "
         << num_games << " games is " << win_rate << endl;

    return 0;
}
