#ifndef SRC_CH07_BITSET_MATRIX_H_
#define SRC_CH07_BITSET_MATRIX_H_

#include <array>
#include <bitset>
#include "maze_state.h"

using MatShape = std::array<std::bitset<W>, H>;

class Mat
{
private:
    MatShape bits_ = MatShape();

    Mat up() const;
    Mat down() const;
    Mat left() const;
    Mat right() const;

public:
    Mat(){};
    Mat(const MatShape &mat) : bits_(mat){};

    bool get(int y, int x) const;
    void set(int y, int x);
    void del(int y, int x);

    void expand();
    void andeq_not(const Mat &mat); // &this&=~mat を１つの演算に
    bool is_equal(const Mat &mat) const;
    bool is_any_equal(const Mat &mat) const;
};

class BitsetState : public WallMazeState
{
private:
    Mat points_mat_ = Mat();
    Mat walls_mat_ = Mat();

public:
    BitsetState(const int seed);
    int get_distance_to_nearest_point() override;
};

#endif
