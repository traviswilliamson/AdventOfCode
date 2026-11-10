#include "server.hpp"
#include "console.hpp"
#include <cpr/cpr.h>
#include <filesystem>
#include <fstream>


serverResponse server::fetchData(unsigned short day) {
    std::filesystem::path inputPath = "input/" + std::to_string(day) + ".txt";
    if (!std::filesystem::exists(inputPath)) {
        std::ifstream cookieFile("input/cookie.txt");
        if (!cookieFile) {
            console::logError("Yo, go make the cookie file!");
            return -1;
        }
        std::string cookie;
        std::getline(cookieFile, cookie);

        // TODO: 2026
        cpr::Response r = cpr::Get(
            cpr::Url{"https://adventofcode.com/2025/day/" + std::to_string(day) + "/input"},
            cpr::Header{{"cookie", cookie},
                        {"User-Agent", "Travis Williamson: "
                                       "https://github.com/traviswilliamson/AdventOfCode"}});

        if (r.status_code != 200) {
            console::logErrorWithHighlight("Got response of ", std::to_string(r.status_code),
                                           " from Advent of Code. Message:");
            console::log(r.text);
            return -1;
        }

        std::ofstream inputFile(inputPath);
        if (!cookieFile) {
            console::logError("Failed to open the input file for writing");
            return -1;
        }

        inputFile << r.text;
        return r.text;
    } else {
        console::logWarning("Input for day " + std::to_string(day) +
                            " already exists, reading cached data");
        std::ifstream inputFile(inputPath, std::ios::binary | std::ios::ate);
        if (!inputFile) {
            console::logError("Failed to open the input file for reading");
            return -1;
        }
        auto size = inputFile.tellg();
        inputFile.seekg(0, std::ios::beg);
        std::string contents;
        contents.resize_and_overwrite(size, [&inputFile](char* buffer, std::size_t n) {
            inputFile.read(buffer, n);
            return n;
        });
        return contents;
    }
}
