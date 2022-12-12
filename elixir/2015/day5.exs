defmodule Day5 do
  defp is_nice_p1(s) do
    String.match?(s, ~r/.*[aeiou].*[aeiou].*[aeiou].*/)
    and String.match?(s, ~r/.*(.)\1.*/)
    and not String.match?(s, ~r/.*(ab|cd|pq|xy).*/)
  end

  defp is_nice_p2(s) do
    String.match?(s, ~r/.*(..).*\1.*/)
    and String.match?(s, ~r/.*(.).\1.*/)
  end


  def part1(s) when length(s) == 0, do: 0
  def part1(s) do
    part1(tl(s)) + if is_nice_p1(hd(s)) do
      1
    else
      0
    end
  end

  def part2(s) when length(s) == 0, do: 0
  def part2(s) do
    part2(tl(s)) + if is_nice_p2(hd(s)) do
      1
    else
      0
    end
  end
end


file_string = File.read!("/home/may/coding/aoc/inputs/2015/day5.txt")
file_strings = String.split(file_string)

IO.puts(Day5.part1(file_strings))
IO.puts(Day5.part2(file_strings))
