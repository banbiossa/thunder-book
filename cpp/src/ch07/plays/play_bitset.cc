
#include <bitset>
#include <iostream>

int main()
{
    using std::cout;
    using std::endl;

    auto bits = std::bitset<3>();

    bits.set(0);
    cout << bits << endl;

    bits <<= 1;
    cout << bits << endl;

    cout << "next" << endl;
    bits = std::bitset<3>(1);
    cout << bits << endl;
    bits <<= 0;
    cout << bits << endl;

    bits = std::bitset<3>(2);
    cout << bits << endl;

    bits = std::bitset<3>(3);
    cout << bits << endl;

    auto other = std::bitset<3>(1);
    cout << bits << " " << other << endl;
    cout << "any" << endl;
    cout << (bits & other).any() << endl;

    cout << "all" << endl;
    cout << (bits ^ other).none() << endl;

    cout << "bits bits not xor all" << endl;
    cout << bits << " " << bits << endl;
    cout << (bits ^ bits) << " " << (bits ^ bits).none() << endl;

    return 0;
}
