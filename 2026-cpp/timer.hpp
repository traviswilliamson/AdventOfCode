#include "console.hpp"
#include <chrono>
#include <format>

struct Timer {
    std::chrono::steady_clock::time_point start;

    Timer() : start(std::chrono::steady_clock::now()) {}

    ~Timer() {
        auto end = std::chrono::steady_clock::now();
        console::logInfo(std::format("Elapsed time: {:%T}", end - start));
    }
};
