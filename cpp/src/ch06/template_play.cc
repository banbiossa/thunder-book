#include "use_template.h"

using std::cout;
using std::endl;

int main()
{

    cout << "int hello" << endl;
    IntHello int_hello = IntHello(3);
    int_hello.say_hello();
    int_hello.something_new();

    cout << "string hello" << endl;
    StringHello string_hello = StringHello("hi");
    string_hello.say_hello();
}
