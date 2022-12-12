defmodule Day2 do
  defp s_to_size(s) do
    [lstr, wstr, hstr] = String.split(s, "x")
    [l, w, h] = [String.to_integer(lstr), String.to_integer(wstr), String.to_integer(hstr)]
    2*(l*w + w*h + h*l) + Enum.min_by([l*w, w*h, h*l], fn x -> x end)
  end

  defp s_to_ribbon_length(s) do
    [lstr, wstr, hstr] = String.split(s, "x")
    [l, w, h] = [String.to_integer(lstr), String.to_integer(wstr), String.to_integer(hstr)]
    2 * Enum.min_by([l+w, w+h, h+l], fn x -> x end) + l*w*h
  end

  def part1(dimensions), do: List.foldl(dimensions, 0, fn s, acc -> acc + s_to_size(s) end)
  def part2(dimensions), do: List.foldl(dimensions, 0, fn s, acc -> acc + s_to_ribbon_length(s) end)
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day2.txt")
file_string_list = String.split(file_string)

IO.puts(Day2.part1(file_string_list))
IO.puts(Day2.part2(file_string_list))
