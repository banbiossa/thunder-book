#include <iostream>
#include <string>
#include "gtest/gtest.h"
#include "src/ch07/maze_state.h"
#include "src/ch07/bitset_single.h"

using namespace std;

class BitsetSingleTest : public ::testing::Test
{
protected:
    SingleMat mat;
    SingleBitsetState state;

    BitsetSingleTest() : mat(), state(0) {}
};

TEST_F(BitsetSingleTest, GetDistanceToNearestPoint)
{
    int actual = state.get_distance_to_nearest_point();
    int expected = 1;
    EXPECT_EQ(actual, expected);
}

TEST_F(BitsetSingleTest, Get)
{
    bool actual = mat.get(0, 0);
    bool expected = false;
    EXPECT_EQ(actual, expected);
}

TEST_F(BitsetSingleTest, Set)
{
    mat.set(0, 0);
    bool actual = mat.get(0, 0);
    bool expected = true;
    EXPECT_EQ(actual, expected);
}

TEST_F(BitsetSingleTest, Del)
{
    mat.set(0, 0);
    mat.del(0, 0);
    bool actual = mat.get(0, 0);
    bool expected = false;
    EXPECT_EQ(actual, expected);
}

TEST_F(BitsetSingleTest, Expand)
{
    mat.set(1, 1);
    mat.expand();
    EXPECT_EQ(mat.get(1, 0), true);
    EXPECT_EQ(mat.get(0, 1), true);
    EXPECT_EQ(mat.get(2, 1), true);
    EXPECT_EQ(mat.get(1, 2), true);
    EXPECT_EQ(mat.get(1, 1), true);
    EXPECT_EQ(mat.get(0, 0), false);
    EXPECT_EQ(mat.get(2, 2), false);
}

TEST_F(BitsetSingleTest, AndEqNot)
{
    SingleMat mat1;
    mat1.set(0, 0);
    mat1.set(0, 1);
    SingleMat mat2;
    mat2.set(0, 1);
    mat1.andeq_not(mat2);
    EXPECT_EQ(mat1.get(0, 0), true);
    EXPECT_EQ(mat1.get(0, 1), false);
}

TEST_F(BitsetSingleTest, IsEqual)
{
    SingleMat mat1;
    mat1.set(0, 0);
    mat1.set(0, 1);
    SingleMat mat2;
    mat2.set(0, 0);
    mat2.set(0, 1);
    EXPECT_EQ(mat1.is_equal(mat2), true);
}

TEST_F(BitsetSingleTest, IsAnyEqual)
{
    SingleMat mat1;
    mat1.set(0, 0);
    mat1.set(0, 1);
    SingleMat mat2;
    mat2.set(0, 0);
    mat2.set(0, 1);
    SingleMat mat3;
    mat3.set(0, 1);
    EXPECT_EQ(mat1.is_any_equal(mat2), true);
    EXPECT_EQ(mat1.is_any_equal(mat3), true);
    EXPECT_EQ(mat2.is_any_equal(mat3), true);
}
