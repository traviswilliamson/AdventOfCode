defmodule Solutions.Day01 do
  def move(line, pos) do
    line
    |> String.split_at(1)
    |> case do
      {"L", count} -> rem(pos - String.to_integer(count), 100)
      {"R", count} -> rem(pos + String.to_integer(count), 100)
    end
  end

  def part_a do
    File.stream!("input/1.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.scan(50, &move/2)
    |> Enum.count(fn x -> x == 0 end)
    |> IO.puts
  end

  def count0(0), do: 1
  def count0(_), do: 0

  def move2(line, {pos, _}) do
    {dir, strcount} = line
    |> String.split_at(1)
    count = String.to_integer(strcount)

    newpos = case dir do
      "L" -> Integer.mod(pos - count, 100)
      "R" -> Integer.mod(pos + count, 100)
    end

    newhits = case dir do
      "L" -> div(count + newpos, 100) + count0(newpos) - count0(pos)
      "R" -> div(count + pos, 100)
    end

    # Was useful for debugging
    # IO.puts [IO.ANSI.white(), Integer.to_string(pos), " ", IO.ANSI.yellow(), line, " ", IO.ANSI.green(), Integer.to_string(newpos), " ", IO.ANSI.cyan(), Integer.to_string(newhits)]

    {newpos, newhits}
  end

  def part_b do
    File.stream!("input/1.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.scan({50, 0}, &move2/2)
    |> Enum.sum_by(fn {_, hits} -> hits end)
    |> IO.puts
  end
end
