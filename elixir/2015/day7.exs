defmodule Day7 do
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
  def part1(s, _idx, map) when length(s) == 0, do: map
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
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day7.txt")
file_strings = String.split(file_string, "\n")

IO.puts(Day7.part1(file_strings))
IO.puts(Day7.part2(file_strings))
