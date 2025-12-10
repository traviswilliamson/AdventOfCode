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

  def intersecting?({{box_a_x, box_a_y}, {box_b_x, box_b_y}}, [{edge_a_x, edge_a_y}, {edge_b_x, edge_b_y}]) do
    !(max(box_a_x, box_b_x) <= min(edge_a_x, edge_b_x) ||
    min(box_a_x, box_b_x) >= max(edge_a_x, edge_b_x) ||
    max(box_a_y, box_b_y) <= min(edge_a_y, edge_b_y) ||
    min(box_a_y, box_b_y) >= max(edge_a_y, edge_b_y))
  end

  def part_b do
    points = for line <- File.stream!("input/9.txt"),
      [x, y] = line |> String.trim |> String.split(","),
      do: {String.to_integer(x), String.to_integer(y)}

    edges = points |> Enum.chunk_every(2, 1, [Enum.at(points, 0)])

    (for {i, i_k} <- points |> Enum.with_index,
      {j, j_k} <- points |> Enum.with_index,
      i_k < j_k, do: {i, j}) # Unique pairs
    |> Stream.filter(fn pair ->
      !(edges
      |> Enum.any?(&(intersecting?(pair, &1))))
    end)
    |> Stream.map(&area/1)
    |> Enum.max
    |> IO.puts
  end
end
