#ifndef SRC_CH08_TIME_KEEPER_H_
#define SRC_CH08_TIME_KEEPER_H_

#include <chrono>

class TimeKeeper
{
private:
    std::chrono::high_resolution_clock::time_point start_time_;
    int64_t time_threshold_;

public:
    TimeKeeper(const int64_t &time_threshold)
        : start_time_(std::chrono::high_resolution_clock::now()),
          time_threshold_(time_threshold) {}
    bool is_time_over() const;
    float get_elapsed_time();
};

#endif
