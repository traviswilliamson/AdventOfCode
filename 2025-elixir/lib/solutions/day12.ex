defmodule Solutions.Day12 do
  def part_a do
    [trees | presents] = File.read!("input/12.txt") |> String.split("\n\n") |> Enum.reverse
    present_sizes = presents |> Enum.reverse
    |> Enum.map(&(String.graphemes(&1) |> Enum.frequencies |> Map.get("#")))
    trees |> String.trim |> String.split("\n") |> Enum.map(fn line ->
      [size | strcounts] = line |> String.split
      counts = strcounts |> Enum.map(&String.to_integer/1)
      [{x, _}, {y, _}] = size |> String.split("x") |> Enum.map(&Integer.parse/1)
      countarea = [counts, present_sizes] |> Stream.zip() |> Stream.map(&Tuple.product/1) |> Enum.sum
      simple_area = div(x, 3) * div(y, 3)
      cond do
        (x * y) < countarea -> :no
        simple_area >= counts |> Enum.sum -> :yes
        true -> :maybe
      end
    end)
    |> Enum.frequencies() |> IO.inspect()
    # Huh. Leaving this here for posterity.
  end

  def part_b do
    IO.puts IO.ANSI.yellow <> "Not implemented" <> IO.ANSI.reset
  end
end
