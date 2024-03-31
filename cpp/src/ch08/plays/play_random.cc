#include <iostream>
#include "src/ch08/random_action.h"
#include "src/ch08/game.h"

int main()
{
    using std::cout;
    using std::endl;

    cout << "random_vs_random" << endl;
    AIFunction actions_wb[2] = {random_action, random_action};
    StateVersion state_versions[2] = {StateVersion::Normal, StateVersion::Normal};
    play_game(actions_wb, state_versions, true);
    return 0;
}
