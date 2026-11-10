#pragma once
#include <iostream>
#include <print>
#include <string>

class console {
    static constexpr std::string_view red = "\033[31m";
    static constexpr std::string_view green = "\033[32m";
    static constexpr std::string_view yellow = "\033[33m";
    static constexpr std::string_view blue = "\033[34m";
    static constexpr std::string_view magenta = "\033[35m";
    static constexpr std::string_view cyan = "\033[36m";
    static constexpr std::string_view reset = "\033[0m";

  public:
    static void logError(std::string message) { std::cerr << red << message << reset << std::endl; }

    static void logErrorWithHighlight(std::string messagePartOne, std::string_view highlight,
                                      std::string messagePartTwo) {
        println(stderr, "{}{}{}{}{}{}{}", red, messagePartOne, yellow, highlight, red,
                messagePartTwo, reset);
    }

    static void logSuccess(std::string message) {
        std::cout << green << message << reset << std::endl;
    }

    static void logWarning(std::string message) {
        std::cout << magenta << message << reset << std::endl;
    }

    static void logInfo(std::string message) { std::cout << cyan << message << reset << std::endl; }

    static void logQuestion(std::string message) {
        std::cout << yellow << message << reset << std::endl;
    }

    static void log(std::string message) { std::cout << message << std::endl; }
};
