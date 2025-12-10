defmodule Solutions.Day07 do
  def beam({pos_row, pos_col}) do
    case Process.get({pos_row + 1, pos_col}) do
      "." ->
        Process.put({pos_row + 1, pos_col}, "|")
        beam({pos_row + 1, pos_col})
      nil -> nil
      "|" -> nil
      "^" ->
        Process.put({pos_row + 1, pos_col}, 1)
        Process.put({pos_row + 1, pos_col + 1}, "|")
        Process.put({pos_row + 1, pos_col - 1}, "|")
        beam({pos_row + 1, pos_col + 1})
        beam({pos_row + 1, pos_col - 1})
    end
  end

  def part_a do
    File.stream!("input/7.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.codepoints/1)
    |> Stream.with_index
    |> Stream.flat_map(fn {chars, line} ->
      chars
      |> Stream.with_index
      |> Enum.map(fn {val, column} -> {{line, column}, val} end)
    end)
    |> Enum.each(fn {key, val} -> Process.put(key, val) end)

    start = Process.get_keys("S") |> Enum.at(0)
    beam(start)
    Process.get |> Stream.map(&elem(&1, 1)) |> Enum.filter(&is_number/1) |> Enum.sum |> IO.puts
  end

  def multi_beam({pos_row, pos_col}) do
    case Process.get({pos_row + 1, pos_col}) do
      "." ->
        newbeams = multi_beam({pos_row + 1, pos_col})
        Process.put({pos_row + 1, pos_col}, newbeams)
        newbeams
      nil -> 1
      "^" ->
        leftbeams = multi_beam({pos_row, pos_col - 1})
        rightbeams = multi_beam({pos_row, pos_col + 1})
        leftbeams + rightbeams
      count when is_number(count) ->
        count
    end
  end

  def part_b do
    File.stream!("input/7.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.codepoints/1)
    |> Stream.with_index
    |> Stream.flat_map(fn {chars, line} ->
      chars
      |> Stream.with_index
      |> Enum.map(fn {val, column} -> {{line, column}, val} end)
    end)
    |> Enum.each(fn {key, val} -> Process.put(key, val) end)

    Process.get_keys("S") |> Enum.at(0)
    |> multi_beam
    |> IO.puts
  end
end
