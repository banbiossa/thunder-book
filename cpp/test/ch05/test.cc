
#include "gtest/gtest.h"

TEST(CH05, ALTERNATE_PS)
{
    int p = 0;
    p ^= 1;
    ASSERT_EQ(p, 1);
    p ^= 1;
    ASSERT_EQ(p, 0);
    p ^= 1;
    ASSERT_EQ(p, 1);
}
