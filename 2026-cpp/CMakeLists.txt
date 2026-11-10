cmake_minimum_required(VERSION 3.10)

project(AdventOfCode)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(cpr CONFIG REQUIRED)

add_executable(AdventOfCode)

target_sources(AdventOfCode PRIVATE
  main.cpp
  server.cpp)

target_link_libraries(AdventOfCode PRIVATE cpr::cpr)
