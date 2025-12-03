defmodule Solutions.Day02 do
  def num_digits(1), do: 1
  def num_digits(int) do
    ceil(:math.log10(int))
  end

  def halve_digits(1), do: 1
  def halve_digits(full) do
    full / (10 ** (num_digits(full) / 2))
    |> floor
  end

  def double_digits(1), do: 11
  def double_digits(pattern) do
    pattern * (10 ** num_digits(pattern)) + pattern
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

  def first_digits(full, k) do
    full
    |> Integer.to_charlist
    |> Enum.take(k)
    |> List.to_integer
  end

  def fill_digits(digit, num) do
    digit
    |> Integer.to_string
    |> String.duplicate(num)
    |> String.to_integer
  end

  def part_b do
    File.read!("input/2.txt")
    |> String.trim()
    |> String.split(",")
    |> Stream.map(&String.split(&1, "-"))
    |> Stream.map(fn [first, second] -> [String.to_integer(first), String.to_integer(second)] end)
    |> Stream.map(fn [start, stop] ->
      1..(floor(num_digits(stop)/2))
      # We don't care about sequences that won't expand to something in our range
      |> Stream.filter(&(rem(num_digits(start), &1) == 0))
      |> Stream.flat_map(fn k ->
        first_digits(start, k)..first_digits(stop, k)
        # Make sure the sequence is repeated at least once (2)
        |> Stream.map(&(fill_digits(&1, max(div(num_digits(start), k), 2))))
        |> Stream.filter(fn i -> i >= start && i <= stop end)
        |> Enum.to_list
      end)
      # Different sequences could expand to the same result
      |> Stream.uniq
      |> Enum.sum
      end)
    |> Enum.sum()
    |> IO.puts()
  end
end
