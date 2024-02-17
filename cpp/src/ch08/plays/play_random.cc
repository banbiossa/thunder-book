#include <iostream>
#include "src/ch08/random.h"
#include "src/ch08/game.h"

int main()
{
    using std::cout;
    using std::endl;

    cout << "random_vs_random" << endl;
    AIFunction actions_wb[2] = {random_action, random_action};
    play_game(actions_wb);
    return 0;
}
