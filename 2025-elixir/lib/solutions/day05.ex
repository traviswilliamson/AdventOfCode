defmodule Solutions.Day05 do
  def part_a do
    [strranges, strvalues] = File.read!("input/5.txt")
    |> String.split("\n\n")
    |> Enum.map(&String.trim/1)

    ranges = for line <- String.split(strranges, "\n"),
      [start, stop] = String.split(line, "-"),
      istart = String.to_integer(start),
      istop = String.to_integer(stop),
      do: istart..istop

    (for strval <- String.split(strvalues, "\n"),
      val = String.to_integer(strval),
      Enum.any?(ranges, fn range -> val in range end),
      do: 1)
    |> Enum.count
    |> IO.puts
  end

  def part_b do
    [strranges, _] = File.read!("input/5.txt")
    |> String.split("\n\n")
    |> Enum.map(&String.trim/1)

    (for line <- String.split(strranges, "\n"),
      [start, stop] = String.split(line, "-"),
      istart = String.to_integer(start),
      istop = String.to_integer(stop),
      do: istart..istop)
    |> Enum.sort(fn l, r -> l.first <= r.first end)
    |> Enum.reduce([..], fn cur, [next | tail] ->
      if Range.disjoint?(cur, next) do
        [cur, next | tail]
      else
        [Range.new(min(cur.first, next.first), max(cur.last, next.last)) | tail]
      end
    end)
    |> Stream.map(&(Range.size(&1)))
    |> Enum.sum
    |> IO.puts
  end
end
