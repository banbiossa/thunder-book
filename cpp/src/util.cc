#include <iostream>
#include <fstream>
#include <cstdarg>
#include "util.h"

void log_to_file(const char *format, ...)
{
    std::ofstream outfile("output.md", std::ios_base::app);
    if (!outfile)
    {
        std::cerr << "Failed to open the log file." << std::endl;
        return;
    }

    va_list args;
    va_start(args, format);

    char buffer[256];
    std::vsnprintf(buffer, sizeof(buffer), format, args);

    va_end(args);

    outfile << buffer << std::endl;
    outfile.close();
}
