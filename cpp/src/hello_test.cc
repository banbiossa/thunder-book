#include <gtest/gtest.h>

TEST(HelloTest, BasicAssertions)
{
    // expect 2 strings
    EXPECT_STRNE("hello", "world");

    EXPECT_EQ(42, 6 * 7);
}
