defmodule Solutions.Day02 do
  def halve_digits(full) do
    full / (10 ** (ceil(:math.log10(full)) / 2))
    |> floor
  end

  def double_digits(1), do: 11
  def double_digits(pattern) do
    pattern * (10 ** ceil(:math.log10(pattern))) + pattern
  end

  def part_a do
    File.read!("input/2.txt")
    |> String.trim()
    |> String.split(",")
    |> Stream.map(&String.split(&1, "-"))
    |> Stream.map(fn [first, second] -> [String.to_integer(first), String.to_integer(second)] end)
    |> Stream.map(fn [start, stop] ->
      halve_digits(start)..halve_digits(stop)
      |> Stream.map(&double_digits/1)
      |> Stream.filter(fn i -> i >= start && i <= stop end)
      |> Enum.sum()
      end)
    |> Enum.sum()
    |> IO.puts()
  end

  def part_b do
    IO.puts IO.ANSI.yellow <> "Not implemented" <> IO.ANSI.reset
  end
end
