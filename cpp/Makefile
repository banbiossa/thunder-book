# Compiler
CXX=clang++

# Compiler flags
CXXFLAGS=-Wall -Wextra -std=c++20

# Build target
TARGET=myprogram

# Source files
SOURCES=$(wildcard *.cpp)

# Object files
OBJECTS=$(SOURCES:.cpp=.o)

# Default rule
all: $(TARGET)
	@./$(TARGET)

echo:
	@echo ${SOURCES}

# Linking rule
$(TARGET): $(OBJECTS)
	@$(CXX) $(OBJECTS) -o $(TARGET)

# Compilation rule
%.o: %.cpp
	@$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean rule
clean:
	rm -f $(OBJECTS) $(TARGET)
