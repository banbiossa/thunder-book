#include <iostream>

using std::cout;
using std::endl;

class Parent
{

public:
    virtual int get() { return 1; };
    void advance() { cout << "advance " << get() << endl; };
};

class Child : public Parent
{
public:
    int get() override { return 2; };
};

int main()
{
    cout << "try child parent behavior" << endl;
    cout << "parent" << endl;
    auto parent = Parent();
    parent.advance();

    cout << "child" << endl;
    auto child = Child();
    child.advance();
}
