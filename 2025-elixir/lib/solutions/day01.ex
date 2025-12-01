defmodule Solutions.Day01 do
  def move(line, pos) do
    line
    |> String.split_at(1)
    |> case do
      {"L", count} -> rem(pos - String.to_integer(count), 100)
      {"R", count} -> rem(pos + String.to_integer(count), 100)
    end
  end

  def part_a do
    File.stream!("input/1.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.scan(50, &move/2)
    |> Enum.count(fn x -> x == 0 end)
    |> IO.puts
  end

  def part_b do
    IO.puts IO.ANSI.yellow <> "Not implemented" <> IO.ANSI.reset
  end
end
