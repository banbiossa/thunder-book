#ifndef SRC_CH07_BITSET_SINGLE_H_
#define SRC_CH07_BITSET_SINGLE_H_

#include <bitset>
#include "maze_state.h"

using Bits = std::bitset<H * W>;

class SingleMat
{
private:
    Bits bits_;
    Bits left_mask_;
    Bits right_mask_;

    SingleMat up() const;
    SingleMat down() const;
    SingleMat right() const;
    SingleMat left() const;

public:
    SingleMat();
    SingleMat(const Bits &single_mat);

    bool get(int y, int x) const;
    void set(int y, int x);
    void del(int y, int x);

    void expand();
    void andeq_not(const SingleMat &mat); // &this&=~mat を１つの演算に
    bool is_equal(const SingleMat &mat) const;
    bool is_any_equal(const SingleMat &mat) const;

    Bits init_left_mask();
    Bits init_right_mask();
};

#endif
