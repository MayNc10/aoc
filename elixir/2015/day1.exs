defmodule Day1 do
  def score_char(s) when s == "(", do: 1
  def score_char(_s),  do: -1

  def part1(s) when length(tl(s)) > 0, do: part1(tl(s)) + score_char(hd(s))
  def part1(s), do: score_char(hd(s))

  def part2(s), do: part2(s, 1, 0)
  def part2(_s, pos, level) when level == -1, do: pos - 1 # Account for extra increment when level reaches -1
  def part2(s, pos, level), do: part2(tl(s), pos + 1, level + score_char(hd(s)))
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day1.txt")
file_string_list = String.codepoints(file_string)

IO.puts(Day1.part1(file_string_list))
IO.puts(Day1.part2(file_string_list))
