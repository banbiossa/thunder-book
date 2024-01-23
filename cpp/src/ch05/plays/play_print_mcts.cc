#include "src/ch05/maze_state.h"
#include "src/ch05/monte_carlo_tree_search.h"

int main()
{
    using std::cout;
    using std::endl;
    auto state = State(0);
    mcts_action(state, 3000, true);
    return 0;
}
