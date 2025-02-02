#include <functional>
#include "maze_state.h"
#include "greedy.h"
#include "beam_search.h"
#include "time_keeper.h"
#include "chokudai_search.h"

using std::cout;
using std::endl;

void test_ai_score(const int game_number,
                   const std::function<int(const MazeState &)> &action_func,
                   const MazeParams &params)
{

    auto time_keeper = TimeKeeper(1000);
    std::mt19937 mt_for_construct(0);
    double score_mean = 0;
    for (int i = 0; i < game_number; i++)
    {
        auto state = MazeState(mt_for_construct(), params);
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
    float elapsed_time = time_keeper.get_elapsed_time() / 1000;
    cout << "Score:\t" << score_mean << ", time\t" << elapsed_time << endl;
}

int main()
{
    const MazeParams params = MazeParams{30, 30, 100};
    int num_games = 100;
    cout << "random" << endl;
    test_ai_score(num_games, random_action, params);

    cout << "greedy" << endl;
    test_ai_score(num_games, greedy_action, params);

    cout << "beam width 2" << endl;
    // define a particial to pass to test_ai_score
    auto partial_beam_search_action = [&](const MazeState &state)
    {
        return beam_search_action(state, 2, params.end_turn_);
    };
    test_ai_score(num_games, partial_beam_search_action, params);

    cout << "beam width 5 with timekeeper 1ms" << endl;
    auto partial_beam_w_time_keeper = [&](const MazeState &state)
    {
        return beam_search_action_with_time_threshold(
            state, /* beam_width */ 5, /* time threshold */ 1);
    };
    test_ai_score(num_games, partial_beam_w_time_keeper, params);

    cout << "beam width 5 with timekeeper 10ms" << endl;
    auto partial_beam_w_time_keeper_10 = [&](const MazeState &state)
    {
        return beam_search_action_with_time_threshold(
            state, /* beam_width */ 5, /* time threshold */ 10);
    };
    test_ai_score(num_games, partial_beam_w_time_keeper_10, params);

    cout << "chokudai search with timekeeper 1ms" << endl;
    auto particial_chokudai_search_with_time_threshold_1 = [&](const MazeState &state)
    {
        return chokudai_search_action_with_time_threshold(
            state, /*beam width */ 1, /* beam_depth */ params.end_turn_, /* time */ 1);
    };
    test_ai_score(num_games, particial_chokudai_search_with_time_threshold_1, params);

    cout << "chokudai search with timekeeper 10ms" << endl;
    auto particial_chokudai_search_with_time_threshold_10 = [&](const MazeState &state)
    {
        return chokudai_search_action_with_time_threshold(
            state, /*beam width */ 1, /* beam_depth */ params.end_turn_, /* time */ 10);
    };
    test_ai_score(num_games, particial_chokudai_search_with_time_threshold_10, params);

    return 0;
}
