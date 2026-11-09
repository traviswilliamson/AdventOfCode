#include <iostream>
#include <fstream>
#include <string>
#include <filesystem>

#include <cpr/cpr.h>

const std::string red = "\033[31m";
const std::string green = "\033[32m";
const std::string yellow = "\033[33m";
const std::string blue = "\033[34m";
const std::string magenta = "\033[35m";
const std::string cyan = "\033[36m";
const std::string reset = "\033[0m";

int main() {
    std::cout << green << "Alright alright alright, here we go" << reset << std::endl;

    // TODO: Move all this somewhere else
    std::ifstream dayFile("input/day.txt");
    if (!dayFile) {
        std::cerr << red << "Yo, go make the day file!" << reset << std::endl;
        return -1;
    }
    std::string day;
    std::getline(dayFile, day);
    std::cout << "day: " << day << std::endl;
    std::size_t sz;
    int intDay = std::stoi(day, &sz);
    char part = day[sz];

    std::filesystem::path inputPath = "input/" + std::to_string(intDay) + ".txt";
    if(!std::filesystem::exists(inputPath))
    {
        std::ifstream cookieFile("input/cookie.txt");
        if (!cookieFile) {
            std::cerr << red << "Yo, go make the cookie file!" << reset << std::endl;
            return -1;
        }
        std::string cookie;
        std::getline(cookieFile, cookie);


        // TODO: 2026
        cpr::Response r = cpr::Get(
            cpr::Url{"https://adventofcode.com/2025/day/" + std::to_string(intDay) + "/input"},
            cpr::Header{{"cookie", cookie},{"User-Agent", "Travis Williamson: https://github.com/traviswilliamson/AdventOfCode"}}
        );

        if (r.status_code != 200)
        {
            std::cerr << red << "Got response of " << yellow << std::to_string(r.status_code) << red << " from Advent of Code" << reset << std::endl;
            std::cerr << red << "Message: " << reset << r.text << std::endl;
            return -1;
        }

        std::ofstream inputFile(inputPath);
        if (!cookieFile) {
            std::cerr << red << "Failed to open the input file for writing" << reset << std::endl;
            return -1;
        }

        inputFile << r.text;
    }
    else
    {
        std::cout << magenta << "Input for day " << std::to_string(intDay) << " already exists" << reset << std::endl;
    }
}
