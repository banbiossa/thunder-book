#include <iostream>
#include "src/ch08/random_action.h"
#include "src/ch08/game.h"
#include "src/ch08/mcts.h"

int main()
{
    using std::cout;
    using std::endl;
    int playout_number = 100;

    AIFunction mcts_f = [&](const ConnectFourState &state)
    {
        return mcts_action(state, playout_number);
    };

    cout << "mcts " << playout_number << " vs_random" << endl;
    AIFunction actions_wb[2] = {mcts_f, random_action};
    play_game(actions_wb);
    return 0;
}
