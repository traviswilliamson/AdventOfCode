orig = File.read!("lib/solutions/day01.ex")

writer = fn i ->
  day = i
  |> Integer.to_string
  |> String.pad_leading(2, "0")

  text = orig
  |> String.replace("01", day)

  File.write!("lib/solutions/day#{day}.ex", text)
end

Enum.each(2..25, writer)
