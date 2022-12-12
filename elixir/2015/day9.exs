defmodule Day9 do
  @max_distance 2 ** 64
  @min_distance 0

  defp add_path(s, map) when length(s) == 0, do: map
  defp add_path(s, map) do
    line = hd(s)
    [path, dis] = String.split(line, " = ")
    dis = String.to_integer(dis)
    [start, dest] = String.split(path, " to ")
    start_map = if(!Map.has_key?(map, start), do: Map.new(), else: map[start])
    start_map = Map.put_new(start_map, dest, dis)
    new_map = Map.put(map, start, start_map)
    # add for other direction
    dest_map = if(!Map.has_key?(map, dest), do: Map.new(), else: map[dest])
    dest_map = Map.put_new(dest_map, start, dis)
    new_map = Map.put(new_map, dest, dest_map)
    add_path(tl(s), new_map)
  end

  defp traverse(locs, _map, _starting_loc) when length(locs) == 0, do: 0
  defp traverse(locs, map, starting_loc) do
    Enum.reduce(locs, @max_distance, fn dest, min_dis ->
      distance = map[starting_loc][dest] + traverse(List.delete(locs, dest), map, dest)
      if(distance < min_dis, do: distance, else: min_dis)
    end)
  end
  defp traverse(locs, map) do
    Enum.reduce(locs,  @max_distance, fn loc, min_dis ->
      dis = traverse(List.delete(locs, loc), map, loc)
      if(dis < min_dis, do: dis, else: min_dis)
    end)
  end

  defp traverse_max(locs, _map, _starting_loc) when length(locs) == 0, do: 0
  defp traverse_max(locs, map, starting_loc) do
    Enum.reduce(locs, @min_distance, fn dest, max_dis ->
      distance = map[starting_loc][dest] + traverse_max(List.delete(locs, dest), map, dest)
      if(distance > max_dis, do: distance, else: max_dis)
    end)
  end
  defp traverse_max(locs, map) do
    Enum.reduce(locs,  @min_distance, fn loc, max_dis ->
      dis = traverse_max(List.delete(locs, loc), map, loc)
      if(dis > max_dis, do: dis, else: max_dis)
    end)
  end


  def part1(s) do
    map = add_path(s, Map.new())
    locations = Map.keys(map)
    traverse(locations, map)
  end

  def part2(s) do
    map = add_path(s, Map.new())
    locations = Map.keys(map)
    traverse_max(locations, map)
  end
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day9.txt")
file_strings = String.split(file_string, "\n")

IO.puts(Day9.part1(file_strings))
IO.puts(Day9.part2(file_strings))
