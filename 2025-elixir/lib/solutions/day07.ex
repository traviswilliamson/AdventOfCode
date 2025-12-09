defmodule Solutions.Day07 do
  def beam(map, {pos_row, pos_col}) do
    case map[{pos_row + 1, pos_col}] do
      "." ->
        map = Map.put(map, {pos_row + 1, pos_col}, "|")
        beam(map, {pos_row + 1, pos_col})
      nil -> map
      "|" -> map
      "^" ->
        map = Map.put(map, {pos_row + 1, pos_col}, 1)
        map = Map.put(map, {pos_row + 1, pos_col + 1}, "|")
        map = Map.put(map, {pos_row + 1, pos_col - 1}, "|")
        map = beam(map, {pos_row + 1, pos_col + 1})
        beam(map, {pos_row + 1, pos_col - 1})
    end
  end

  def part_a do
    grid = File.stream!("input/7.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.codepoints/1)
    |> Stream.with_index
    |> Stream.flat_map(fn {chars, line} ->
      chars
      |> Stream.with_index
      |> Enum.map(fn {val, column} -> {{line, column}, val} end)
    end)
    |> Enum.into(%{})

    start = Enum.find(grid, fn {_, val} -> val == "S" end) |> elem(0)
    grid = beam(grid, start)
    grid |> Map.values |> Enum.filter(&is_number/1) |> Enum.sum |> IO.puts
  end

  def multi_beam(map, {pos_row, pos_col}) do
    case map[{pos_row + 1, pos_col}] do
      "." ->
        {map, newbeams} = multi_beam(map, {pos_row + 1, pos_col})
        map = Map.put(map, {pos_row + 1, pos_col}, newbeams)
        {map, newbeams}
      nil -> {map, 1}
      "^" ->
        {map, leftbeams} = multi_beam(map, {pos_row, pos_col - 1})
        {map, rightbeams} = multi_beam(map, {pos_row, pos_col + 1})
        {map, leftbeams + rightbeams}
      count when is_number(count) ->
        {map, count}
    end
  end

  def part_b do
    grid = File.stream!("input/7.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.codepoints/1)
    |> Stream.with_index
    |> Stream.flat_map(fn {chars, line} ->
      chars
      |> Stream.with_index
      |> Enum.map(fn {val, column} -> {{line, column}, val} end)
    end)
    |> Enum.into(%{})

    start = Enum.find(grid, fn {_, val} -> val == "S" end) |> elem(0)
    {_, count} = multi_beam(grid, start)
    IO.puts count
  end
end
