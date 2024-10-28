# 2024 Advent of Code - Rust
Directory for Travis Williamson's 2024 [Rust](https://www.rust-lang.org/) solutions for [Advent of Code](https://adventofcode.com/).

## Setup
1. Install [Rust](https://www.rust-lang.org/tools/install)
1. Install [c++ build tools](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup#install-visual-studio-recommended-or-the-microsoft-c-build-tools).
1. Install VSCode recommended extensions
1. (Optional) For communication to Advent of Code servers, create a `Cookie.txt` file in the `input` folder and add your own cookie that gets created when logging into the Advent of Code website. While on the Advent of Code website, if you open the Network tab in your browser's Dev Tools, you'll see the cookie in the header of API calls that are made while navigating the site. This typically expires after a month so you'll need to update it each year.
1. Set the current day in a `day.txt` file in the `input` folder in the format `ddf`, where `dd` is the 2 digit day, and `f` is `"a"` or `"b"`, whether this is for the first or second part. Example: `05b`
1. To run, call `cargo run` from this folder.