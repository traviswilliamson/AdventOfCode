defmodule Solutions.Day09 do
  def area({{a_x, a_y}, {b_x, b_y}}) do
    (abs(a_x - b_x) + 1) * (abs(a_y - b_y) + 1)
  end

  def part_a do
    points = for line <- File.stream!("input/9.txt"),
      [x, y] = line |> String.trim |> String.split(","),
      do: {String.to_integer(x), String.to_integer(y)}

    (for {i, i_k} <- points |> Enum.with_index,
      {j, j_k} <- points |> Enum.with_index,
      i_k < j_k, do: {i, j}) # Unique pairs
    |> Stream.map(&area/1)
    |> Enum.max
    |> IO.puts
  end

  def part_b do
    IO.puts IO.ANSI.yellow <> "Not implemented" <> IO.ANSI.reset
  end
end
