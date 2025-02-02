#include "bitset_single.h"

bool SingleMat::get(int y, int x) const
{
    return bits_[y * H + x];
}

void SingleMat::set(int y, int x)
{
    bits_.set(y * H + x);
}

void SingleMat::del(int y, int x)
{
    bits_.reset(y * H + x);
}

Bits SingleMat::up() const
{
    return bits_ >> W;
}

Bits SingleMat::down() const
{

    return bits_ << W;
}

Bits SingleMat::left() const
{
    return (bits_ & left_mask_) >> 1;
}

Bits SingleMat::right() const
{
    return (bits_ & right_mask_) << 1;
}

Bits SingleMat::init_left_mask()
{
    Bits mask = Bits();
    for (int y = 0; y < H; y++)
        mask |= Bits(1) << (y * W);
    mask = ~mask;
    return mask;
}

Bits SingleMat::init_right_mask()
{
    Bits mask = Bits();
    for (int y = 0; y < H; y++)
        mask |= Bits(1) << (y * W + W - 1);
    mask = ~mask;
    return mask;
}

SingleMat::SingleMat(const Bits &single_mat)
{
    bits_ = single_mat;
    left_mask_ = init_left_mask();
    right_mask_ = init_right_mask();
};

SingleMat::SingleMat()
{
    bits_ = Bits();
    left_mask_ = init_left_mask();
    right_mask_ = init_right_mask();
};

void SingleMat::expand()
{
    // avoid writing in place
    SingleMat mat = *this;
    mat.bits_ |= up();
    mat.bits_ |= down();
    mat.bits_ |= left();
    mat.bits_ |= right();

    bits_ = mat.bits_;
}

void SingleMat::andeq_not(const SingleMat &mat)
{
    bits_ &= ~mat.bits_;
}

bool SingleMat::is_equal(const SingleMat &mat) const
{
    return bits_ == mat.bits_;
}

bool SingleMat::is_any_equal(const SingleMat &mat) const
{
    return (bits_ & mat.bits_).any();
}

SingleBitsetState::SingleBitsetState(const int seed) : State(seed)
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

int SingleBitsetState::get_distance_to_nearest_point()
{
    auto mat = SingleMat();
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

std::shared_ptr<State> SingleBitsetState::clone() const
{
    return std::make_shared<SingleBitsetState>(*this);
}
