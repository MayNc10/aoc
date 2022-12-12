defmodule Day4 do
  defp check_hash(hash) when length(hash) == 1, do: hd(hash) == "0"
  defp check_hash(hash) do
    if hd(hash) != "0" do
      false
    else
      check_hash(tl(hash))
    end
  end

  def part1(s), do: part1(s, 0)
  defp part1(s, idx) do
    hash_str = :crypto.hash(:md5, s <> to_string(idx))
    hash = Base.encode16(hash_str)
    hash_front = String.slice(hash, 0..4)

    if check_hash(String.codepoints(hash_front)) do
      idx
    else
      part1(s, idx + 1)
    end
  end

  def part2(s), do: part2(s, 0)
  defp part2(s, idx) do
    hash_str = :crypto.hash(:md5, s <> to_string(idx))
    hash = Base.encode16(hash_str)
    hash_front = String.slice(hash, 0..5)

    if check_hash(String.codepoints(hash_front)) do
      idx
    else
      part2(s, idx + 1)
    end
  end
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day4.txt")

IO.puts(Day4.part1(file_string))
IO.puts(Day4.part2(file_string))
