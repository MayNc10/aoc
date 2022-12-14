defmodule Aoc2015 do
  @moduledoc """
  This module solves all problems from AoC 2015.
  It also has a main function, meaning it can be built as a script and ran to solve every problem.
  Just remember to change the paths for each problem!
  """
  defmodule Day1 do
    @moduledoc """
    This module solves both parts of the day 1 problem from AoC 2015
    """
    def score_char(s) when s == "(", do: 1
    def score_char(_s),  do: -1

    def part1(s) when length(tl(s)) > 0, do: part1(tl(s)) + score_char(hd(s))
    def part1(s), do: score_char(hd(s))

    def part2(s), do: part2(s, 1, 0)
    def part2(_s, pos, level) when level == -1, do: pos - 1 # Account for extra increment when level reaches -1
    def part2(s, pos, level), do: part2(tl(s), pos + 1, level + score_char(hd(s)))
    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day1.txt")
      file_string_list = String.codepoints(file_string)

      IO.puts(part1(file_string_list))
      IO.puts(part2(file_string_list))
    end
  end
  defmodule Day2 do
    @moduledoc """
    This module solves both parts of the day 2 problem from AoC 2015
    """
    defp s_to_size(s) do
      [lstr, wstr, hstr] = String.split(s, "x")
      [l, w, h] = [String.to_integer(lstr), String.to_integer(wstr), String.to_integer(hstr)]
      2 * (l * w + w * h + h * l) + Enum.min_by([l * w, w * h, h * l], fn x -> x end)
    end

    defp s_to_ribbon_length(s) do
      [lstr, wstr, hstr] = String.split(s, "x")
      [l, w, h] = [String.to_integer(lstr), String.to_integer(wstr), String.to_integer(hstr)]
      2 * Enum.min_by([l + w, w + h, h + l], fn x -> x end) + l * w * h
    end

    def part1(dimensions), do: List.foldl(dimensions, 0, fn s, acc -> acc + s_to_size(s) end)
    def part2(dimensions), do: List.foldl(dimensions, 0, fn s, acc -> acc + s_to_ribbon_length(s) end)
    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day2.txt")
      file_string_list = String.split(file_string)

      IO.puts(part1(file_string_list))
      IO.puts(part2(file_string_list))
    end
  end
  defmodule Day3 do
    @moduledoc """
    This module solves both parts of the day 3 problem from AoC 2015
    """
    defp deliver_presents(s) when s == [], do: {MapSet.new(), {0, 0}}
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
    defp deliver_presents_robo(s, map, pos_set, is_santa_turn) when s == [] do
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
    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day3.txt")
      file_string_list = String.codepoints(file_string)

      IO.puts(part1(file_string_list))
      IO.puts(part2(file_string_list))
    end
  end
  defmodule Day4 do
    @moduledoc """
    This module solves both parts of the day 4 problem from AoC 2015
    """
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
    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day4.txt")
      IO.puts(part1(file_string))
      IO.puts(part2(file_string))
    end
  end
  defmodule Day5 do
    @moduledoc """
    This module solves both parts of the day 5 problem from AoC 2015
    """
    defp is_nice_p1(s) do
      String.match?(s, ~r/.*[aeiou].*[aeiou].*[aeiou].*/)
      and String.match?(s, ~r/.*(.)\1.*/)
      and not String.match?(s, ~r/.*(ab|cd|pq|xy).*/)
    end

    defp is_nice_p2(s) do
      String.match?(s, ~r/.*(..).*\1.*/)
      and String.match?(s, ~r/.*(.).\1.*/)
    end


    def part1(s) when s == [], do: 0
    def part1(s) do
      part1(tl(s)) + if is_nice_p1(hd(s)) do
        1
      else
        0
      end
    end

    def part2(s) when s == [], do: 0
    def part2(s) do
      part2(tl(s)) + if is_nice_p2(hd(s)) do
        1
      else
        0
      end
    end
    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day5.txt")
      file_strings = String.split(file_string)

      IO.puts(part1(file_strings))
      IO.puts(part2(file_strings))
    end
  end
  defmodule Day6 do
    @moduledoc """
    This module solves both parts of the day 6 problem from AoC 2015
    """
    defp light_area_p1(s, lights) when s == [], do: lights
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

    defp light_area_p2(s, lights) when s == [], do: lights
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
    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day6.txt")
      file_strings = String.split(file_string, "\n")

      IO.puts(part1(file_strings))
      IO.puts(part2(file_strings))
    end
  end
  defmodule Day7 do
    @moduledoc """
    This module solves both parts of the day 7 problem from AoC 2015
    """
    defp resolve_ref(ref, map) do
      if Integer.parse(ref) == :error do
        if Map.has_key?(map, ref) do
          Map.fetch!(map, ref)
        else
          :error
        end
      else
        String.to_integer(ref)
      end
    end

    defp process_line(s, map) do
      import Bitwise
      # Process into tokens
      [inputs, output] = String.split(s, " -> ")
      if(Map.has_key?(map, output), do: throw({map, true}))

      inputs = String.split(inputs)
      res = case length(inputs) do
        # Just a straight ref
        1 -> resolve_ref(Enum.at(inputs, 0), map)
        # NOT x, invert
        2 ->
          arg = resolve_ref(Enum.at(inputs, 1), map)
          if(arg != :error, do: ~~~arg, else: :error)
        # Lots of different statements here
        3 ->
          [arg1, op, arg2] = inputs
          arg1 = resolve_ref(arg1, map)
          arg2 = resolve_ref(arg2, map)
          if arg1 == :error or arg2 == :error do
            :error
          else
            case op do
              "AND" -> arg1 &&& arg2
              "OR" -> arg1 ||| arg2
              "RSHIFT" -> arg1 >>> arg2
              "LSHIFT" -> arg1 <<< arg2
            end
          end
      end
      if res != :error do
        {Map.put(map, output, res), true}
      else
        {map, false}
      end
    catch
      value -> value
    end

    def part1(s) do
      map = part1(s, 0, Map.new())
      Map.fetch!(map, "a")
    end
    def part1(s, _idx, map) when s == [], do: map
    def part1(s, idx, map) do
      {new_map, success} = process_line(Enum.at(s, idx), map)
      if success do
        part1(List.delete_at(s, idx), 0, new_map)
      else
        part1(s, idx + 1, new_map)
      end
    end

    def part2(s) do
      a = part1(s)
      new_map = %{"b" => a}
      new_map = part1(s, 0, new_map)
      Map.fetch!(new_map, "a")
    end

    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day7.txt")
      file_strings = String.split(file_string, "\n")

      IO.puts(part1(file_strings))
      IO.puts(part2(file_strings))
    end
  end
  defmodule Day8 do
    @moduledoc """
    This module solves both parts of the day 8 problem from AoC 2015
    """
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

    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day8.txt")
      file_strings = String.split(file_string, "\n")

      IO.puts(part1(file_strings))
      IO.puts(part2(file_strings))
    end
  end
  defmodule Day9 do
    @moduledoc """
    This module solves both parts of the day 9 problem from AoC 2015
    """
    @max_distance 2 ** 64
    @min_distance 0

    defp add_path(s, map) when s == [], do: map
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

    defp traverse(locs, _map, _starting_loc) when locs == [], do: 0
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

    defp traverse_max(locs, _map, _starting_loc) when locs == [], do: 0
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

    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day9.txt")
      file_strings = String.split(file_string, "\n")

      IO.puts(part1(file_strings))
      IO.puts(part2(file_strings))
    end
  end
  defmodule Day10 do
    @moduledoc """
    This module solves both parts of the day 10 problem from AoC 2015
    """
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

    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day10.txt")
      file_strings = String.codepoints(file_string)

      IO.puts(part1(file_strings))
      IO.puts(part2(file_strings))
    end
  end
  defmodule Day11 do
    @moduledoc """
    This module solves both parts of the day 11 problem from AoC 2015
    """
    def is_valid_password(s) do
      if String.contains?(s, ["i", "o", "l"]) do
        false
      else
        s_list = String.codepoints(s)
        if !Enum.reduce(0..length(s_list) - 4, false, fn idx, bool ->
          <<this_char_ascii::utf8>> = Enum.at(s_list, idx)
          <<next_char_ascii::utf8>> = Enum.at(s_list, idx + 1)
          <<next_next_char_ascii::utf8>> = Enum.at(s_list, idx + 2)
          bool or (next_char_ascii - this_char_ascii == 1 and next_next_char_ascii - next_char_ascii == 1)
        end) do
          false
        else
          {_, res} = Enum.reduce(Enum.chunk_every(s_list, 2, 1), {:nil, false}, fn chars, combined ->
            {first, found} = combined
            if found do
              {first, found}
            else
              match = Enum.at(chars, 0) == Enum.at(chars, 1)
              case {first == :nil, match} do
                {true, true} -> {chars, false}
                {true, false} -> {:nil, false}
                {false, true} -> {first, first != chars}
                {false, false} -> {first, false}
              end
            end
          end)
          res
        end
      end
    end

    def part1(s) do
      #IO.puts(List.to_string(s))
      # compute next password
      {next_s, _} = Enum.reduce(Enum.reverse(s), {[], 1}, fn char, combined ->
        {list, carry} = combined
        <<ascii::utf8>> = char
        ascii = ascii - 97
        ascii = ascii + carry
        next_carry = div(ascii, 26)
        next_char_ascii = rem(ascii, 26) + 97
        next_char = List.to_string([next_char_ascii])
        {[next_char | list], next_carry}
      end)
      next_s = List.to_string(next_s)
      if(is_valid_password(next_s), do: next_s, else: part1(String.codepoints(next_s)))
    end

    def part2(s), do: part1(String.codepoints(part1(s)))

    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day11.txt")
      file_strings = String.codepoints(file_string)

      IO.puts(part1(file_strings))
      IO.puts(part2(file_strings))
    end
  end
  defmodule Day12 do
    @moduledoc """
    This module solves both parts of the day 12 problem from AoC 2015
    """
    def part1(s) do
      s = String.replace(s, ~r/[a-zA-Z:{}""\[\],]/, " ")
      s = String.split(s)
      Enum.reduce(s, 0, fn substr, acc ->
        acc + String.to_integer(substr)
      end)
    end

    defp sum_result(result) do
      if is_list(result) do
        list = result
        Enum.reduce(list, 0, fn elem, acc ->
          if is_integer(elem) do
            acc + elem
          else
            if is_bitstring(elem) do
              acc
            else
              acc + sum_result(elem)
            end
          end
        end)
      else
        map = result
        if Enum.count(Map.values(map), fn x -> is_bitstring(x) and x == "red" end) > 0 do
          0
        else
          Enum.reduce(Map.values(map), 0, fn elem, acc ->
            if is_integer(elem) do
              acc + elem
            else
              if is_bitstring(elem) do
                acc
              else
                acc + sum_result(elem)
              end
            end
          end)
        end
      end
    end

    def part2(s) do
      import Jason
      {_, result} = Jason.decode(s)
      sum_result(result)
    end

    def run() do
      file_string = File.read!("/home/may/coding/aoc/inputs/2015/day12.txt")
      #file_strings = String.codepoints(file_string)

      IO.puts(part1(file_string))
      #IO.puts(part2(file_strings))
      IO.puts(part2(file_string))
    end
  end
  def main([]) do
    #Day1.run()
    #Day2.run()
    #Day3.run()
    #Day4.run()
    #Day5.run()
    #Day6.run()
    #Day7.run()
    #Day8.run()
    #Day9.run()
    #Day10.run()
    #Day11.run()
    Day12.run()
  end
end
