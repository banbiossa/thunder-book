#ifndef SRC_CH06_TRY_TEMPLATE_H_
#define SRC_CH06_TRY_TEMPLATE_H_

#include <iostream>
#include <typeinfo>

template <typename T>
class Hello
{
protected:
    T value;

public:
    Hello(T value) : value(value){};
    virtual ~Hello() = default;
    void say_hello();

    virtual void something_new() = 0;
};

template <typename T>
void Hello<T>::say_hello()
{
    using std::cout;
    using std::endl;
    cout << "hello my value is " << value
         << " the type is " << typeid(value).name() << endl;
};

#endif
