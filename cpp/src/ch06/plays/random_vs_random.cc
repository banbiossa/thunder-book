#include "src/ch06/game.h"
#include "src/ch06/random_action.h"

using std::cout;
using std::endl;

int main()
{
    AIFunction actions_wb[2] = {random_action, random_action};

    // play black and white
    int num_games = 1000;
    float win_rate = games_black_and_white(num_games, actions_wb, /* print every */ 100);

    cout << "random vs random over "
         << num_games << " games is " << win_rate << endl;

    return 0;
}
