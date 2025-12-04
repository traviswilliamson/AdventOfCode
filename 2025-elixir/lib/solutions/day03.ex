defmodule Solutions.Day03 do
  def part_a do
    File.stream!("input/3.txt")
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.codepoints/1)
    |> Stream.map(fn chars -> Enum.map(chars, &String.to_integer/1) end)
    |> Stream.map(&Enum.with_index/1)
    |> Stream.map(fn intlist ->
      {first_digit, first_index} = Enum.max_by(intlist |> Enum.slice(0..-2//1), &elem(&1, 0))
      {second_digit, _} = Enum.max_by(intlist |> Enum.slice((first_index+1)..-1//1), &elem(&1, 0))
      (first_digit * 10) + second_digit
    end)
    |> Enum.sum
    |> IO.puts
  end

  def part_b do
    IO.puts IO.ANSI.yellow <> "Not implemented" <> IO.ANSI.reset
  end
end
