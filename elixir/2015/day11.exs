defmodule Day11 do
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
end

file_string = File.read!("/home/may/coding/aoc/inputs/2015/day11.txt")
file_strings = String.codepoints(file_string)

IO.puts(Day11.part1(file_strings))
IO.puts(Day11.part2(file_strings))
