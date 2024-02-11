#ifndef SRC_CH07_GLOBALS_H_
#define SRC_CH07_GLOBALS_H_

#include <string>
#include <unordered_set>

extern std::unordered_set<std::string> hash_check_db;
void call(std::string signature, std::string message = "hello");

#endif
