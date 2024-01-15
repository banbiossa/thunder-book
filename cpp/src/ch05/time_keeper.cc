
#include "time_keeper.h"

bool TimeKeeper::is_time_over() const
{
    using std::chrono::duration_cast;
    using std::chrono::milliseconds;

    auto diff = std::chrono::high_resolution_clock::now() - this->start_time_;
    return duration_cast<milliseconds>(diff).count() >= time_threshold_;
}

float TimeKeeper::get_elapsed_time()
{
    using std::chrono::duration_cast;
    using std::chrono::milliseconds;

    auto diff = std::chrono::high_resolution_clock::now() - this->start_time_;
    return duration_cast<milliseconds>(diff).count();
}
