#include <iostream>
#include <filesystem>

const std::string red = "\033[31m";
const std::string green = "\033[32m";
const std::string yellow = "\033[33m";
const std::string blue = "\033[34m";
const std::string magenta = "\033[35m";
const std::string cyan = "\033[36m";
const std::string reset = "\033[0m";

namespace fs = std::filesystem;

int main() {
    std::cout << "Alright alright alright, here we go" << std::endl;
    std::cout << red << "red " << green << "green " << yellow << "yellow " << blue << "blue " << magenta << "magenta " << cyan << "cyan " << reset << "done " << std::endl;
    for (const auto& entry : fs::directory_iterator(".")) {
        if (fs::is_regular_file(entry.status())) {
            std::cout << "File: " << entry.path().filename() << "\n";
        } else if (fs::is_directory(entry.status())) {
            std::cout << "Dir:  " << entry.path().filename() << "\n";
        }
    }

}
