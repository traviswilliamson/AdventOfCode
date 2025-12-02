defmodule ServerIO do
  def download_input_file(day) when is_number(day) do
    unless File.exists? "input/#{day}.txt" do
      Req.get!("https://adventofcode.com/2025/day/#{day}/input", headers: %{cookie: File.read!("input/cookie.txt"), user_agent: "Travis Williamson: https://github.com/traviswilliamson/AdventOfCode" }, into: File.stream!("input/#{day}.txt"))
      IO.puts [IO.ANSI.green(), "Downloaded input", IO.ANSI.reset()]
    else
      IO.puts [IO.ANSI.magenta(), "Input already downloaded", IO.ANSI.reset()]
    end
  end
end
