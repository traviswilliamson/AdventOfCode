# 2025 Advent Of Code - Elixir
Directory for Travis Williamson's 2024 [Elixir](https://https://elixir-lang.org/) solutions for [Advent of Code](https://adventofcode.com/).

## Installation
1. Install [Elixir](https://hexdocs.pm/elixir/introduction.html#installation)
1. Add Elixir to path, restart to take effect
1. Install hex, `mix local.hex`
1. For VSCode ElixirLS VSCode extension
1. (Optional) For communication to Advent of Code servers, create a `Cookie.txt` file in the `input` folder and add your own cookie that gets created when logging into the Advent of Code website. While on the Advent of Code website, if you open the Network tab in your browser's Dev Tools, you'll see the cookie in the header of API calls that are made while navigating the site. This typically expires after a month so you'll need to update it each year.
1. Set the current day in a `day.txt` file in the `input` folder in the format `ddf`, where `dd` is the 2 digit day, and `f` is `"a"` or `"b"`, whether this is for the first or second part. Example: `05b`
1. To run interactively, call `iex -S mix`
1. Now call the start function