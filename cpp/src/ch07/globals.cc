#include <iostream>
#include "globals.h"

std::unordered_set<std::string> hash_check_db;

void call(std::string signature, std::string message)
{
    if (hash_check_db.count(signature) > 0)
        return;
    hash_check_db.insert(signature);
    std::cout << signature << " says " << message << std::endl;
}
