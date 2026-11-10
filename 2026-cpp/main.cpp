#include "console.hpp"
#include "day.hpp"
#include "server.hpp"
#include "timer.hpp"
#include <string>

int main() {
    console::logSuccess("Alright alright alright, here we go");

    auto day = day::read();

    auto res = server::fetchData(day.day);
    if (!res.success) {
        return -1;
    }

    Timer timer; // On exit, will print elapsed time
    // Todo: Run day
}
