#include <iostream>
#include <fstream>
#include <string>

#include <cpr/cpr.h>

const std::string red = "\033[31m";
const std::string green = "\033[32m";
const std::string yellow = "\033[33m";
const std::string blue = "\033[34m";
const std::string magenta = "\033[35m";
const std::string cyan = "\033[36m";
const std::string reset = "\033[0m";

int main() {
    std::cout << "Alright alright alright, here we go" << std::endl;

    // TODO: Move all this somewhere else
    std::ifstream cookieFile("input/cookie.txt");
    if (!cookieFile) {
        std::cerr << red << "Yo, go make the cookie file!\n";
        return -1;
    }
    std::string cookie;
    std::getline(cookieFile, cookie);

    std::ifstream dayFile("input/cookie.txt");
    if (!dayFile) {
        std::cerr << red << "Yo, go make the day file!\n";
        return -1;
    }
    std::string day;
    std::getline(dayFile, cookie);

    httplib::Client cli("https://adventofcode.com");
    // TODO: 2026
    auto res = cli.Get("/2025/day" + day + "/input", httplib::Headers{
        {"cookie", cookie},
        {"User-Agent", "Travis Williamson: https://github.com/traviswilliamson/AdventOfCode"}
    });

    cpr::Response r = cpr::Get(
        cpr::Url{"https://adventofcode.com/2025/day" + std::to_string(day) + "/input"},
        cpr::Authentication{"user", "pass", cpr::AuthMode::BASIC}
    );
    std::cout << r.status_code << std::endl;
    std::cout << r.text << std::endl;
}
