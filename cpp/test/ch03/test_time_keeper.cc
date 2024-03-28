#include <chrono>
#include <thread>
#include "gtest/gtest.h"
#include "src/ch03/time_keeper.h"

TEST(TimeKeeperTest, IsTimeOver)
{
    TimeKeeper time_keeper(1);
    EXPECT_FALSE(time_keeper.is_time_over());
    // Sleep for 1 ms
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
    EXPECT_TRUE(time_keeper.is_time_over());
}

TEST(TimeKeeperTest, GetElapsedTime)
{
    // this looks very fragile
    TimeKeeper time_keeper(1);
    EXPECT_TRUE(time_keeper.get_elapsed_time() == 0);
    // Sleep for 1 ms
    std::this_thread::sleep_for(std::chrono::milliseconds(1));
    EXPECT_TRUE(time_keeper.get_elapsed_time() == 1);
}
