#pragma once
#include "console.hpp"
#include <cstdlib>
#include <fstream>
#include <iostream>

struct dayData {
    unsigned short day;
    bool partOne;
    dayData(unsigned short d, char part) : day(d), partOne(part == 'a') {}
};

class day {
  public:
    static dayData read() {
        std::ifstream dayFile("input/day.txt");
        if (!dayFile) {
            console::logError("Yo, go make the day file!");
            exit(-1);
        }
        std::string day;
        std::getline(dayFile, day);
        console::logInfo("Loading day: " + day);
        if (day.length() != 3) {
            console::logError("Invalid day input, must be length 3");
            exit(-2);
        }
        std::size_t sz;
        int intDay = std::stoi(day, &sz);
        if (intDay <= 0 || intDay > 12) {
            console::logError("Invalid day input, must begin with an int 1-12");
            exit(-3);
        }
        char part = day[sz];
        if (part != 'a' && part != 'b') {
            console::logError("Invalid day input, must end in 'a' or 'b'");
            exit(-4);
        }

        return dayData(intDay, part);
    }
};
