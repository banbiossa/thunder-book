#include <functional>
#include "maze-state.h"
#include "greedy.h"

using std::cout;
using std::endl;

void test_ai_score(const int game_number, const std::function<int(const MazeState &)> &action_func)
{

    std::mt19937 mt_for_construct(0);
    double score_mean = 0;
    for (int i = 0; i < game_number; i++)
    {
        auto state = MazeState(mt_for_construct());
        while (!state.is_done())
        {
            // pass in the action_func (like greedy_action or random_action)
            // from an argument
            state.advance(action_func(state));
        }
        auto score = state.game_score_;
        score_mean += score;
    }
    score_mean /= (double)game_number;
    cout << "Score:\t" << score_mean << endl;
}

int main()
{
    int num_games = 100;
    cout << "random" << endl;
    test_ai_score(num_games, random_action);

    cout << "greedy" << endl;
    test_ai_score(num_games, greedy_action);
    return 0;
}
