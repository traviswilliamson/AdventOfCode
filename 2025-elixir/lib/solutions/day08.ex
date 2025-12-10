defmodule Solutions.Day08 do
  def distance({[a_x, a_y, a_z], [b_x, b_y, b_z]}) do
    :math.sqrt((b_x - a_x) ** 2 + (b_y - a_y) ** 2 + (b_z - a_z) ** 2)
  end

  def part_a do
    vertices = File.stream!("input/8.txt")
    |> Enum.map(fn line -> line |> String.trim
      |> String.split(",") |> Enum.map(&String.to_integer/1)
    end)

    (for {i, i_k} <- vertices |> Enum.with_index,
      {j, j_k} <- vertices |> Enum.with_index,
      i_k < j_k, do: {i, j}) # Unique edges
    |> Enum.sort_by(&distance/1) # Sorted
    |> Enum.slice(0..999) # First 1k
    |> Enum.each(fn {edge_start, edge_stop} ->
      case {Process.get(edge_start), Process.get(edge_stop)} do
        {nil, nil} -> # New subgraph
          Process.put(edge_start, edge_start)
          Process.put(edge_stop, edge_start)
        {head, nil} -> # 1 existing subgraph
          Process.put(edge_stop, head)
        {nil, head} -> # 1 existing subgraph
          Process.put(edge_start, head)
        {head_a, head_a} -> # Already part of the same subgraph, nothin' to do
          nil
        {head_a, head_b} -> # 2 existing subgraphs, time to merge
          Process.get_keys(head_b) |> Enum.each(&(Process.put(&1, head_a)))
      end
    end)

    Process.get
    |> Enum.group_by(&(elem(&1, 1)))
    |> Enum.map(&(Enum.count(elem(&1, 1))))
    |> Enum.sort(:desc)
    |> Stream.take(3) # READ THE INSTRUCTIONS, TRAVIS
    |> Enum.product
    |> IO.puts
  end

  def part_b do
    vertices = File.stream!("input/8.txt")
    |> Enum.map(fn line -> line |> String.trim
      |> String.split(",") |> Enum.map(&String.to_integer/1)
    end)

    (for {i, i_k} <- vertices |> Enum.with_index,
      {j, j_k} <- vertices |> Enum.with_index,
      i_k < j_k, do: {i, j}) # Unique edges
    |> Enum.sort_by(&distance/1) # Sorted
    |> Enum.each(fn {edge_start, edge_stop} ->
      case {Process.get(edge_start), Process.get(edge_stop)} do
        {nil, nil} -> # New subgraph
          Process.put(edge_start, edge_start)
          Process.put(edge_stop, edge_start)
        {head, nil} -> # 1 existing subgraph
          Process.put(edge_stop, head)
          Process.put(:merged, Enum.at(edge_start, 0) * Enum.at(edge_stop, 0)) # Distance from wall
        {nil, head} -> # 1 existing subgraph
          Process.put(edge_start, head)
          Process.put(:merged, Enum.at(edge_start, 0) * Enum.at(edge_stop, 0)) # Distance from wall
        {head_a, head_a} -> # Already part of the same subgraph, nothin' to do
          nil
        {head_a, head_b} -> # 2 existing subgraphs, time to merge
          Process.get_keys(head_b) |> Enum.each(&(Process.put(&1, head_a)))
          Process.put(:merged, Enum.at(edge_start, 0) * Enum.at(edge_stop, 0)) # Distance from wall
      end
    end)

    Process.get(:merged) |> IO.puts
  end
end
