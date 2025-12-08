defmodule Solutions.Day06 do
  def parse(text) do
    case Integer.parse(text) do
      {num, ""} -> num
      :error -> case text do
        "+" -> &Kernel.+/2
        "*" -> &Kernel.*/2
      end
    end
  end

  def part_a do
    (for line <- File.stream!("input/6.txt"),
      vals = String.split(line),
      intvals = (for val <- vals, do: parse(val)),
      do: intvals)
    |> Enum.zip_with(&Function.identity/1) # Transposes matrix
    |> Stream.map(&Enum.reverse/1) # Get the operand first
    |> Stream.map(fn [op | vals] -> Enum.reduce(vals, op) end)
    |> Enum.sum
    |> IO.puts
  end

  def part_b do
    File.read!("input/6.txt")
    |> String.split("\n")
    |> Enum.slice(0..-2//1) # Get rid of the empty last line
    |> Stream.map(&String.reverse/1) # We're gonna work backwards
    |> Stream.map(&String.codepoints/1) # Grab those sweet, sweet codepoints
    |> Stream.zip_with(&Function.identity/1) # Transposes matrix
    |> Stream.filter(&!Enum.all?(&1, fn c -> c == " " end)) # Get rid of the gap between problems
    |> Enum.reduce([[0]], fn row, [current | answers] ->
      {num, _} = row |> Enum.slice(0..-2//1) |> List.to_string |> String.trim |> Integer.parse
      problem = case current do
        [0] -> [num] # [0] is our placeholder problem
        _ -> [num | current]
      end
      case Enum.at(row, -1) do # Solve?
        " " -> [problem | answers] # Solve later
        "*" -> [[0], Enum.reduce(problem, &Kernel.*/2) | answers]
        "+" -> [[0], Enum.reduce(problem, &Kernel.+/2) | answers]
      end
    end)
    |> Enum.slice(1..-1//1) # Get rid of that [0] we prepended
    |> Enum.sum
    |> IO.puts
  end
end
