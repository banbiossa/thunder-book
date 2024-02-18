#include "gtest/gtest.h"
#include "src/ch08/try_bits.h"

TEST(CH08, FLOOR_BIT)
{
    int64_t actual = floor_bit(3, 3);
    int64_t expected = 0b000100010001ULL;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, FLOOR_BIT2)
{
    int64_t actual = floor_bit(3, 4);
    int64_t expected = 0b000010000100001ULL;
    ASSERT_EQ(actual, expected);
}
