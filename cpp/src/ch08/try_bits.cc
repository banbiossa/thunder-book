#include "try_bits.h"

uint64_t floor_bit(int w, int h)
{
    uint64_t bit = 0;
    for (int x = 0; x < w; x++)
        bit |= 1ULL << x * (h + 1);
    return bit;
}

uint64_t ones(int w)
{
    return (1ULL << w) - 1;
}

uint64_t head_filled(int w, int h)
{
    // 0b10001000
    uint64_t bit = 0;
    for (int x = 0; x < w; x++)
    {
        bit |= 1ULL << (h - 1 + x * h);
    }
    return bit;
}

uint64_t head_missing_bit(int w, int h)
{
    // 0b011011
    /*
    1 << h = 100
    100 - 1 = 011
    011 << x*(h+1) = 011000
    の組み合わせ
    */
    uint64_t bit = 0;
    for (int x = 0; x < w; x++)
    {
        bit |= ((1ULL << h) - 1) << x * (h + 1);
    }
    return bit;
}
