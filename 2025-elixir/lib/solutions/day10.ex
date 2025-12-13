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

  def min_power_buttons(machine) do
    # Linear algebra time
    tensors = for tensor_i <- 0..((machine.jolts |> Enum.count) - 1),
      freqs = machine.buttons |> Enum.map(&Enum.frequencies/1),
      do: freqs |> Enum.map(&(Map.get(&1, tensor_i, 0)))
    Nx.LinAlg.least_squares(tensors |> Nx.tensor, machine.jolts |> Map.values |> Nx.tensor)
    |> Nx.to_flat_list |> Enum.sum |> IO.inspect
  end

  def part_b do
    File.stream!("input/10.txt")
    |> Stream.map(&parse/1)
    |> Stream.map(&min_power_buttons/1)
    |> Enum.sum
    |> IO.puts
  end
end

# 16931 too high
