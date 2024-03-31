#include "gtest/gtest.h"
#include "src/ch08/try_bits.h"

TEST(CH08, FLOOR_BIT)
{
    uint64_t actual = floor_bit(3, 3);
    uint64_t expected = 0b000100010001ULL;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, FLOOR_BIT2)
{
    uint64_t actual = floor_bit(3, 4);
    uint64_t expected = 0b000010000100001ULL;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, ONES)
{
    uint64_t actual = ones(3);
    uint64_t expected = 0b111;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, HEAD_FIILED)
{
    uint64_t actual = head_filled(1, 3);
    uint64_t expected = 0b100;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, HEAD_FIILED2)
{
    uint64_t actual = head_filled(2, 3);
    uint64_t expected = 0b100100;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, HEAD_FIILED4)
{
    uint64_t actual = head_filled(4, 3);
    uint64_t expected = 0b100100100100;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, HEAD_MISSING)
{
    uint64_t actual = head_missing_bit(1, 2);
    uint64_t expected = 0b011;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, HEAD_MISSING2)
{
    uint64_t actual = head_missing_bit(2, 2);
    uint64_t expected = 0b011011;
    ASSERT_EQ(actual, expected);
}

TEST(CH08, HEAD_MISSING3)
{
    uint64_t actual = head_missing_bit(3, 2);
    uint64_t expected = 0b011011011;
    ASSERT_EQ(actual, expected);
}
