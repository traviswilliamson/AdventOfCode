defmodule Main do
  use Application

  def start(_type, _args) do
    {day, type} = File.read!("input/day.txt")
    |> Integer.parse
    ServerIO.download_input_file(day)
    # TODO: Run code for the day

    # Normally a supervision tree. We're just exiting.
    Task.start(fn -> IO.puts IO.ANSI.cyan() <> "Done!" end)
  end
end
