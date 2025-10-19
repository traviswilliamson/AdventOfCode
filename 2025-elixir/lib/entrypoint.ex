defmodule Main do
  use Application

  def start(_type, _args) do
    {day, type} = File.read!("input/day.txt")
    |> String.trim
    |> String.split_at(2)

    day
    |> String.to_integer
    |> ServerIO.download_input_file

    apply(String.to_atom("Elixir.Solutions.Day#{day}"), String.to_atom("part_#{type}"), [])

    # Normally a supervision tree. We're just exiting.
    Task.start(fn -> IO.puts IO.ANSI.cyan() <> "Done!" end)
  end
end
