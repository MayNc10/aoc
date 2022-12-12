defmodule Day3 do
  defp deliver_presents(s) when length(s) == 0, do: {MapSet.new(), {0,0}}
  defp deliver_presents(s) do
    {map, pos} = deliver_presents(tl(s))
    newpos = case hd(s) do
      "^" -> {elem(pos, 0) + 1, elem(pos, 1)}
      "v" -> {elem(pos, 0) - 1, elem(pos, 1)}
      ">" -> {elem(pos, 0), elem(pos, 1) + 1}
      "<" -> {elem(pos, 0), elem(pos, 1) - 1}
    end
    {MapSet.put(map, newpos), newpos}
  end

  defp deliver_presents_robo(s), do: deliver_presents_robo(s, MapSet.new(), {0, 0, 0, 0}, true)
  defp deliver_presents_robo(s, map, pos_set, is_santa_turn) when length(s) == 0 do
    idx_base = if is_santa_turn do
      0
    else
      2
    end
    MapSet.put(map, {elem(pos_set, idx_base), elem(pos_set, idx_base + 1)})
  end
  defp deliver_presents_robo(s, map, pos_set, is_santa_turn) do
    idx_base = if is_santa_turn do
      0
    else
      2
    end
    pos = {elem(pos_set, idx_base), elem(pos_set, idx_base + 1)}
    new_map = MapSet.put(map, pos)
    new_pos_set = case hd(s) do
      "^" -> put_elem(pos_set, idx_base, elem(pos_set, idx_base) + 1)
      "v" -> put_elem(pos_set, idx_base, elem(pos_set, idx_base) - 1)
      ">" -> put_elem(pos_set, idx_base + 1, elem(pos_set, idx_base + 1) + 1)
      "<" -> put_elem(pos_set, idx_base + 1, elem(pos_set, idx_base + 1) - 1)
    end
    deliver_presents_robo(tl(s), new_map, new_pos_set, !is_santa_turn)
  end

  def part1(s), do: MapSet.size(elem(deliver_presents(s), 0))

  def part2(s), do: MapSet.size(deliver_presents_robo(s))
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day3.txt")
file_string_list = String.codepoints(file_string)

IO.puts(Day3.part1(file_string_list))
IO.puts(Day3.part2(file_string_list))
