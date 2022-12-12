defmodule Day6 do
  defp light_area_p1(s, lights) when length(s) == 0, do: lights
  defp light_area_p1(s, lights) do
    line = hd(s)
    {area, instruction} = case String.slice(line, 0..6) do
      "toggle " -> {String.slice(line, 7..-1), "toggle"}
      "turn on" -> {String.slice(line, 8..-1), "on"}
      "turn of" -> {String.slice(line, 9..-1), "off"}
    end
    area = for coords <- String.split(area, " through ") do
      for i <- String.split(coords, ",") do
        String.to_integer(i)
      end
    end
    [[row_min, col_min], [row_max, col_max]] = area

    new_lights = Enum.reduce(row_min..row_max, lights, fn row, lights_inner ->
      Enum.reduce(col_min..col_max, lights_inner, fn col, map ->
        init_val = map[{row, col}]
        Map.replace!(map, {row, col}, case instruction do
          "toggle" -> !init_val
          "on" -> true
          "off" -> false
        end)
      end)
    end)
    light_area_p1(tl(s), new_lights)
  end

  defp light_area_p2(s, lights) when length(s) == 0, do: lights
  defp light_area_p2(s, lights) do
    line = hd(s)
    {area, instruction} = case String.slice(line, 0..6) do
      "toggle " -> {String.slice(line, 7..-1), "toggle"}
      "turn on" -> {String.slice(line, 8..-1), "on"}
      "turn of" -> {String.slice(line, 9..-1), "off"}
    end
    area = for coords <- String.split(area, " through ") do
      for i <- String.split(coords, ",") do
        String.to_integer(i)
      end
    end
    [[row_min, col_min], [row_max, col_max]] = area

    new_lights = Enum.reduce(row_min..row_max, lights, fn row, lights_inner ->
      Enum.reduce(col_min..col_max, lights_inner, fn col, map ->
        init_val = map[{row, col}]
        Map.replace!(map, {row, col}, case instruction do
          "toggle" -> init_val + 2
          "on" -> init_val + 1
          "off" -> if(init_val > 0, do: init_val - 1, else: 0)
        end)
      end)
    end)
    light_area_p2(tl(s), new_lights)
  end

  def part1(s) do
    lights = Enum.reduce(0..999, %{}, fn row, map ->
      Enum.reduce(0..999, map, fn col, map ->
        Map.put(map, {row, col}, false)
      end)
    end)

    lights = light_area_p1(s, lights)
    lights_flat = Map.values(lights)
    List.foldl(lights_flat, 0, fn x, acc -> acc + if(x, do: 1, else: 0) end)
  end

  def part2(s) do
    lights = Enum.reduce(0..999, %{}, fn row, map ->
      Enum.reduce(0..999, map, fn col, map ->
        Map.put(map, {row, col}, 0)
      end)
    end)

    lights = light_area_p2(s, lights)
    lights_flat = Map.values(lights)
    List.foldl(lights_flat, 0, fn x, acc -> acc + x end)
  end

end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day6.txt")
file_strings = String.split(file_string, "\n")

IO.puts(Day6.part1(file_strings))
IO.puts(Day6.part2(file_strings))
