defmodule Solutions.Day11 do
  def search(src, dest) when src == dest, do: 1
  def search(src, dest) do
    Process.get(src) |> Stream.map(&(search(&1, dest)))
    |> Enum.sum
  end

  def part_a do
    File.stream!("input/11.txt")
    |> Enum.each(fn line ->
      [src, tar] = line |> String.trim |> String.split(": ")
      Process.put(src, tar |> String.split(" "))
    end)
    search("you", "out") |> IO.puts
  end

  def cond_search(src, dest, f, d) when src == dest do
    case f && d do
      true -> 1
      false -> 0
    end
  end
  def cond_search(src, dest, f, d) do
    f = f || src == "fft"
    d = d || src == "dac"
    case Process.get({src, f, d}) do
      sum when is_integer(sum) -> sum
      nil ->
        sum = Process.get(src) |> Stream.map(&(cond_search(&1, dest, f, d)))
        |> Enum.sum
        Process.put({src, f, d}, sum)
        sum
    end

  end

  def part_b do
    File.stream!("input/11.txt")
    |> Enum.each(fn line ->
      [src, tar] = line |> String.trim |> String.split(": ")
      Process.put(src, tar |> String.split(" "))
    end)
    cond_search("svr", "out", false, false) |> IO.puts
  end
end
