#ifndef SRC_CH06_USE_TEMPLATE_H_
#define SRC_CH06_USE_TEMPLATE_H_

#include "try_template.h"

class IntHello : public Hello<int>
{
public:
    IntHello(int value) : Hello(value){};
    void something_new() override;
};

class StringHello : public Hello<std::string>
{
public:
    StringHello(std::string value) : Hello(value){};
    void something_new() override;
};

#endif
