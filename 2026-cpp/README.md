# 2026 Advent Of Code - C++
Directory for Travis Williamson's 2026 [c++23](https://cppreference.com/cpp/23) solutions for [Advent of Code](https://adventofcode.com/).

Projects at work won't be upgrading to this version for a while, and I wanted to play with the new features.

## Installation
1. Install [clang](https://github.com/llvm/llvm-project/releases) & add to path. Make sure clang knows where to access the standard library headers. If the build spits out errors about not knowing where packages like `<iostream>` are, it can't find the standard headers.
1. Install [cmake](https://cmake.org/download/)
1. Install & setup [vcpkg](https://learn.microsoft.com/en-us/vcpkg/get_started/get-started?pivots=shell-bash#1---set-up-vcpkg)
1. Install [ninja](https://github.com/ninja-build/ninja/wiki/Pre-built-Ninja-packages)
1. `cmake --preset debug`
1. `cmake --preset release`
1. (Optional) For communication to Advent of Code servers, create a `Cookie.txt` file in the `input` folder and add your own cookie that gets created when logging into the Advent of Code website. While on the Advent of Code website, if you open the Network tab in your browser's Dev Tools, you'll see the cookie in the header of API calls that are made while navigating the site. This typically expires after a month so you'll need to update it each year.

## Running
1. Set the current day in a `day.txt` file in the `input` folder in the format `ddf`, where `dd` is the 2 digit day, and `f` is `"a"` or `"b"`, whether this is for the first or second part. Example: `05b`
1. Build and run
    1. In zed, use the relevant run tasks
    1. Or `cmake --build --preset release`, then run it `build/release/AdventOfCode.exe`

## Debugging
1. CodeLLDB debugging is set up as a zed debug task

## TODO
1. Run logic based on day
