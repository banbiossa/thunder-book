#include <gtest/gtest.h>
#include <fstream>
#include <sstream>

// Include the header file containing the log_to_file function
#include "src/util.h"

TEST(LogToFileTest, LogString)
{
    // Remove the log file if it exists
    std::remove("output.md");

    // Call the log_to_file function with a string
    log_to_file("Hello, World!");

    // Open the log file for reading
    std::ifstream infile("output.md");
    ASSERT_TRUE(infile.is_open()) << "Failed to open the log file.";

    // Read the contents of the log file
    std::string content((std::istreambuf_iterator<char>(infile)),
                        (std::istreambuf_iterator<char>()));

    // Verify the logged content
    EXPECT_EQ(content, "Hello, World!\n");

    // Close the log file
    infile.close();
}

TEST(LogToFileTest, LogFormatString)
{
    // Remove the log file if it exists
    std::remove("output.md");

    // Call the log_to_file function with a format string and arguments
    log_to_file("The value of x is %d and y is %.2f.", 10, 3.14);

    // Open the log file for reading
    std::ifstream infile("output.md");
    ASSERT_TRUE(infile.is_open()) << "Failed to open the log file.";

    // Read the contents of the log file
    std::string content((std::istreambuf_iterator<char>(infile)),
                        (std::istreambuf_iterator<char>()));

    // Verify the logged content
    EXPECT_EQ(content, "The value of x is 10 and y is 3.14.\n");

    // Close the log file
    infile.close();
}
