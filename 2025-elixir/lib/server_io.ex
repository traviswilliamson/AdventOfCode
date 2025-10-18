defmodule ServerIO do
  def download_input_file(day) when is_number(day) do
    Req.get!("https://adventofcode.com/2025/day/#{day}/input", headers: %{cookie: File.read!("input/cookie.txt")}, into: File.stream!("input/#{day}.txt"))
  end
end
