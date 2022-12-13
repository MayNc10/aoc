defmodule Day10 do
  @part_1_iterations 40
  @part_2_iterations 50

  defp listmap_to_map(listmap) do
    keys = Enum.sort(Map.keys(listmap))
    {map, _, _} = Enum.reduce(keys, {Map.new(), 0, -1}, fn key, combined ->
      num = listmap[key]
      {map, idx, last_num} = combined
      if num != last_num do
        new_idx = idx + 1
        {Map.put_new(map, new_idx, {num, 1}), new_idx, num}
      else
        {elem(Map.get_and_update!(map, idx, fn {num, count} -> {{num, count}, {num, count + 1}} end), 1), idx, last_num}
      end
    end)
    map
  end

  defp list_to_listmap(list) do
    Enum.reduce(0..length(list) - 1, %{}, fn idx, map ->
      Map.put_new(map, idx, Enum.at(list, idx))
    end)
  end

  defp map_to_listmap(map) do
    keys = Enum.sort(Map.keys(map))

    {listmap, _} = Enum.reduce(keys, {Map.new(), 0}, fn key, {listmap, idx} ->
      {num, count} = map[key]
      new_map = Map.put_new(listmap, idx, count)
      new_map = Map.put_new(new_map, idx + 1, num)
      {new_map, idx + 2}
    end)
    listmap
  end

  defp list_to_string(list) do
    Enum.reduce(list, "", fn num, acc ->
      acc <> Integer.to_string(num)
    end)
  end

  def part1(list) do
    list = for n <- list, do: String.to_integer(n)
    listmap = list_to_listmap(list)
    #IO.inspect(list_to_string(listmap_to_list(listmap)))

    listmap = Enum.reduce(1..@part_1_iterations, listmap, fn iter, listmap ->
      IO.puts(iter)
      new_listmap = map_to_listmap(listmap_to_map(listmap))
      #IO.inspect(list_to_string(listmap_to_list(new_listmap)))
      new_listmap
    end)
    length(Map.keys(listmap))
  end

  def part2(list) do
    list = for n <- list, do: String.to_integer(n)
    listmap = list_to_listmap(list)
    #IO.inspect(list_to_string(listmap_to_list(listmap)))
    listmap = Enum.reduce(1..@part_2_iterations, listmap, fn iter, listmap ->
      #IO.puts(iter)
      new_listmap = map_to_listmap(listmap_to_map(listmap))
      #IO.inspect(list_to_string(listmap_to_list(new_listmap)))
      new_listmap
    end)
    length(Map.keys(listmap))
  end
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day10.txt")
file_strings = String.codepoints(file_string)

IO.puts(Day10.part1(file_strings))
IO.puts(Day10.part2(file_strings))
