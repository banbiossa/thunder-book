#include "test_speed.h"
#include "alpha_beta.h"
#include "mini_max.h"
#include "maze_state.h"

int main()
{
    using std::cout;
    using std::endl;

    auto states = get_sample_states(100);
    calculate_execution_speed(
        StringAIPair("alpha_beta", [](const State &state)
                     { return alpha_beta_action(state, END_TURN); }),
        states);
    calculate_execution_speed(
        StringAIPair("mini_max", [](const State &state)
                     { return mini_max_action(state, END_TURN); }),
        states);
    return 0;
}
