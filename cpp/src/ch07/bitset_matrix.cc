#include "bitset_matrix.h"

bool Mat::get(int y, int x) const
{
    return bits_[y][x];
}

void Mat::set(int y, int x)
{
    bits_[y].set(x);
}

void Mat::del(int y, int x)
{
    bits_[y].reset(x);
}

Mat Mat::up() const
{
    Mat mat = *this;
    for (int y = 0; y < H - 1; y++)
        mat.bits_[y] |= mat.bits_[y + 1];
    return mat;
}

Mat Mat::down() const
{
    Mat mat = *this;
    for (int y = H - 1; y >= 1; y--)
        mat.bits_[y] |= mat.bits_[y - 1];
    return mat;
}

Mat Mat::left() const
{
    Mat mat = *this;
    for (int y = 0; y < H; y++)
        mat.bits_[y] >>= 1;
    return mat;
}

Mat Mat::right() const
{
    Mat mat = *this;
    for (int y = 0; y < H; y++)
        mat.bits_[y] <<= 1;
    return mat;
}

void Mat::expand()
{
    Mat m_up = up();
    Mat m_down = down();
    Mat m_left = left();
    Mat m_right = right();
    for (int y = 0; y < H; y++)
    {
        bits_[y] |= m_up.bits_[y];
        bits_[y] |= m_down.bits_[y];
        bits_[y] |= m_left.bits_[y];
        bits_[y] |= m_right.bits_[y];
    }
}

void Mat::andeq_not(const Mat &mat)
{
    for (int y = 0; y < H; y++)
        bits_[y] &= ~mat.bits_[y];
}

bool Mat::is_equal(const Mat &mat) const
{
    for (int y = 0; y < H; y++)
        if (bits_[y] != mat.bits_[y])
            return false;
    return true;
}

bool Mat::is_any_equal(const Mat &mat) const
{
    for (int y = 0; y < H; y++)
        if ((bits_[y] & mat.bits_[y]).any())
            return true;
    return false;
}

BitsetState::BitsetState(const int seed) : WallMazeState(seed)
{
    for (int y = 0; y < H; y++)
    {
        for (int x = 0; x < W; x++)
        {
            if (walls_[y][x])
                walls_mat_.set(y, x);
            if (points_[y][x])
                points_mat_.set(y, x);
        }
    }
}

#include <iostream>

namespace
{
    int count_bitset = 0;

    void counter()
    {
        if (count_bitset == 0)
        {
            std::cout << "inside matrix" << std::endl;
            count_bitset++;
        }
    }
}

int BitsetState::get_distance_to_nearest_point()
{
    counter();
    auto mat = Mat();
    mat.set(character_.y_, character_.x_);
    for (int depth = 0;; ++depth)
    {
        // ポイントに触れているか
        if (mat.is_any_equal(points_mat_))
            return depth;

        auto prev = mat;
        mat.expand();
        mat.andeq_not(walls_mat_);
        // break if nothing changes
        if (mat.is_equal(prev))
            break;
    }
    return H * W;
}
