#include "try_bits.h"

uint64_t floor_bit(int w, int h)
{
    uint64_t bit = 0;
    for (int x = 0; x < w; x++)
        bit |= 1ULL << x * (h + 1);
    return bit;
}
