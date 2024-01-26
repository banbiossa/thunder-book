#include "src/ch06/game.h"
#include "src/ch06/random_action.h"
#include "src/ch06/monte_carlo.h"

int main()
{
    int playout_number = 1000;
    AIFunction monte_carlo_f = [&](const State &state, int player_id)
    {
        return primitive_monte_carlo_action(state, player_id, playout_number);
    };
    AIFunction actions_wb[2] = {monte_carlo_f, random_action};

    // play black and white
    return 0;
}
