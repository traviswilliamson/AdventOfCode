defmodule Solutions.Day04 do
  def charToVal("."), do: 0
  def charToVal("@"), do: 1

  def checkNeighbors(map, {{coord_i, coord_j}, _}) do
    (for i <- -1..1, j <- -1..1, !(i == 0 && j == 0), do: {i, j}, into: [])
    |> Stream.map(fn {i, j} -> {coord_i + i, coord_j + j} end)
    |> Stream.map(fn coord -> map[coord] end)
    |> Stream.filter(&is_number/1)
    |> Enum.sum
  end

  def part_a do
    grid = File.stream!("input/4.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.codepoints/1)
    |> Stream.with_index
    |> Stream.flat_map(fn {chars, line} ->
      Stream.map(chars, &charToVal/1)
      |> Stream.with_index
      |> Enum.map(fn {val, column} -> {{line, column}, val} end)
    end)
    |> Enum.into(%{})

    grid
    |> Stream.filter(fn {_, val} -> val == 1 end)
    |> Stream.map(&checkNeighbors(grid, &1))
    |> Stream.filter(&(&1 < 4))
    |> Enum.count
    |> IO.puts
  end

  def recursiveCheck(map) when map == %{}, do: 0
  def recursiveCheck(map) do
    removed_map = map
    |> Stream.filter(fn coord -> checkNeighbors(map, coord) >= 4 end)
    |> Enum.into(%{})

    num_removed = Enum.count(map) - Enum.count(removed_map)
    if num_removed == 0 do
      0
    else
      num_removed + recursiveCheck(removed_map)
    end
  end

  def part_b do
    grid = File.stream!("input/4.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.codepoints/1)
    |> Stream.with_index
    |> Stream.flat_map(fn {chars, line} ->
      Stream.map(chars, &charToVal/1)
      |> Stream.with_index
      |> Enum.map(fn {val, column} -> {{line, column}, val} end)
    end)
    |> Stream.filter(fn {_, val} -> val == 1 end)
    |> Enum.into(%{})

    recursiveCheck(grid)
    |> IO.puts
  end
end
