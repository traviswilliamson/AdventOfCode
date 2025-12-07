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
    IO.puts IO.ANSI.yellow <> "Not implemented" <> IO.ANSI.reset
  end
end
