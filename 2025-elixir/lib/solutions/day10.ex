defmodule Solutions.Day10 do
  require Integer
  def parse(line) do
    [strgoal | rest] = line |> String.trim |> String.split
    [strjolts | strbuttons] = rest |> Enum.reverse
    goal = strgoal |> String.slice(1..-2//1) |> String.codepoints |> Stream.map(&(&1 == "#")) |> Stream.with_index |> Enum.into(%{}, fn {b, i} -> {i, b} end)
    buttons = strbuttons |> Enum.reverse |> Enum.map(fn set ->
      set |> String.slice(1..-2//1) |> String.split(",") |> Enum.map(&String.to_integer/1)
    end)
    jolts = strjolts |> String.slice(1..-2//1) |> String.split(",") |> Stream.map(&String.to_integer/1) |> Stream.with_index |> Enum.into(%{}, fn {b, i} -> {i, b} end)
    %{goal: goal, buttons: buttons, jolts: jolts}
  end

  def comb(0, _), do: [[]]
  def comb(_, []), do: []
  def comb(m, [h|t]) do
    (for l <- comb(m-1, t), do: [h|l]) ++ comb(m, t)
  end

  def activate_satisfy(button_set, goal) do
    hit_counts = button_set |> List.flatten |> Enum.frequencies()
    goal |> Enum.all?(fn {i, truthy} -> (Map.get(hit_counts, i, 0) |> Integer.is_odd) == truthy end)
  end

  def min_activate_buttons(machine) do # A button is pressed at most once
    0..Enum.count(machine.buttons) |> Enum.find_index(fn count ->
      comb(count, machine.buttons) |> Enum.any?(&(activate_satisfy(&1, machine.goal)))
    end)
  end

  def part_a do
    File.stream!("input/10.txt")
    |> Stream.map(&parse/1)
    |> Stream.map(&min_activate_buttons/1)
    |> Enum.sum
    |> IO.puts
  end

  def min_joltage(counts) do
    cond do
      counts |> Enum.all?(fn {_, v} -> v == 0 end) -> 0 # We're done!
      counts |> Enum.any?(fn {_, v} -> v < 0 end) -> :infinity # However we got here wasn't valid
      true ->
        case Process.get(counts) do
          :infinity -> :infinity # No solutions down this path
          cached_count when is_number(cached_count) -> cached_count
          nil ->
            # In the end, we'll need combos that can get us this odd number
            arity = 0..(Process.get(:num_machines) - 1)
            |> Enum.map(fn i -> {i, Bitwise.band(Map.get(counts, i), 1) == 1} end)

            solution = case Process.get(arity) do
              nil ->
                # At most 1-each button presses that can get us that
                satisfying_combos = Process.get(:all_combos) |> Enum.filter(&(activate_satisfy(&1, arity)))
                Process.put(arity, satisfying_combos) # Keep these cached, we'll see them a lot
                satisfying_combos
              list when is_list(list) -> list
            end |> case do
              [] -> :infinity # Couldn't solve this one
              valid_combos ->
                # Recursion time!
                valid_combos |> Stream.map(fn combo ->
                  combo_counts = combo |> List.flatten |> Enum.frequencies
                  # Now that we've got even remainig joltage to get, halve the problem and do it again
                  counts |> Stream.map(fn {k, v} ->
                    {k, div((v - Map.get(combo_counts, k, 0)), 2)} end)
                    |> Enum.into(%{})
                  |> min_joltage |> case do
                    :infinity -> :infinity # Bad solution
                    valid -> valid * 2 + Enum.count(combo)
                  end
                end) |> Enum.min
            end
            Process.put(counts, solution) # Remember what we found
            solution
        end
    end
  end

  def handle_line(machine) do
    Process.put(:num_machines, machine.goal |> Enum.count)
    all_combos = 0..Enum.count(machine.buttons) |> Enum.flat_map(&comb(&1, machine.buttons))
    Process.put(:all_combos, all_combos)
    min_joltage(machine.jolts)
  end

  def part_b do
    File.stream!("input/10.txt")
    |> Stream.map(&parse/1)
    |> Task.async_stream(&handle_line/1, timeout: 20000)
    |> Enum.sum_by(fn {:ok, res} -> res end)
    |> IO.puts
  end
end
