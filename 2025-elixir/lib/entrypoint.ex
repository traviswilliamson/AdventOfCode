defmodule Main do
  use Application

  def start(_type, _args) do
    # TODO: Get day
    ServerIO.download_input_file(1)
    # TODO: Run code for the day
    Task.start(fn -> IO.puts IO.ANSI.cyan() <> "Done!" end)
  end
end
