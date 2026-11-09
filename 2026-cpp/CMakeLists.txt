cmake_minimum_required(VERSION 3.10)

project(AdventOfCode)
set(CMAKE_CXX_STANDARD 23)

find_package(cpr CONFIG REQUIRED)

add_executable(AdventOfCode main.cpp)

target_link_libraries(AdventOfCode PRIVATE cpr::cpr)
