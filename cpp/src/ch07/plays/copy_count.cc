#include <random>
#include <queue>
#include <iostream>

int operator_count = 0;

class State
{
public:
    int value_;

    State(const int value = 0) : value_(value) {}
    State &operator=(const State & /*state*/)
    {
        operator_count++;
        return *this;
    }
    State(const State &) = default;
};

bool operator<(const State &state1, const State &state2)
{
    return state1.value_ < state2.value_;
}

int main()
{
    using std::cout;
    using std::endl;

    std::mt19937 mt(1);
    std::priority_queue<State> que;

    for (int i = 0; i < 100; i++)
        que.push(State(mt() % 100));

    cout << "operator is called " << operator_count
         << " times" << endl;

    return 0;
}
