defmodule Day8 do
 def part1(s) do
  length_raw_strings = Enum.reduce(s, 0, fn line, acc -> String.length(line) + acc end)
  length_processed_strings = Enum.reduce(s, 0, fn line, acc ->
    line = String.slice(line, 1..-2)
    processed_line = Regex.replace(~r/(\\x..)|(\\.)/, line, ".")
    String.length(processed_line) + acc
  end)
  length_raw_strings - length_processed_strings
 end

 def part2(s) do
  length_raw_strings = Enum.reduce(s, 0, fn line, acc -> String.length(line) + acc end)
  length_processed_strings = Enum.reduce(s, 0, fn line, acc ->
    processed_line = String.replace(line, ["\"", "\\"], fn <<char>> -> <<char>> <> <<char>> end)
    String.length(processed_line) + 2 + acc # add extra quotes
  end)
  length_processed_strings - length_raw_strings
 end
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day8.txt")
file_strings = String.split(file_string, "\n")

IO.puts(Day8.part1(file_strings))
IO.puts(Day8.part2(file_strings))
