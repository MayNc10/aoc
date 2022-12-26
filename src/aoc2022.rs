use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day9;
pub mod day8;
pub mod day10; 
pub mod day11; 
pub mod day12; 
pub mod day13; 
pub mod day14; 
pub mod day15; 
pub mod day16; 
pub mod day17; 
pub mod day18; 
pub mod day19; 
pub mod day20; 
pub mod day21; 
pub mod day22; 
pub mod day23; 
pub mod day24; 
pub mod day25; 



pub fn main() { 
    day1::day1(fs::read_to_string("./inputs/2022/day1.txt").unwrap().as_str());
    day2::day2(fs::read_to_string("./inputs/2022/day2.txt").unwrap().as_str());
    day3::day3(fs::read_to_string("./inputs/2022/day3.txt").unwrap().as_str());
    day4::day4(fs::read_to_string("./inputs/2022/day4.txt").unwrap().as_str());
    day5::day5(fs::read_to_string("./inputs/2022/day5.txt").unwrap().as_str());
    day6::day6(fs::read_to_string("./inputs/2022/day6.txt").unwrap().as_str());
    day7::day7(fs::read_to_string("./inputs/2022/day7.txt").unwrap().as_str());
    day8::day8(fs::read_to_string("./inputs/2022/day8.txt").unwrap().as_str());
    day9::day9(fs::read_to_string("./inputs/2022/day9.txt").unwrap().as_str());
    day10::day10(fs::read_to_string("./inputs/2022/day10.txt").unwrap().as_str());
    day11::day11(fs::read_to_string("./inputs/2022/day11.txt").unwrap().as_str());
    day12::day12(fs::read_to_string("./inputs/2022/day12.txt").unwrap().as_str());
    day13::day13(fs::read_to_string("./inputs/2022/day13.txt").unwrap().as_str());
    day14::day14(fs::read_to_string("./inputs/2022/day14.txt").unwrap().as_str());
    day15::day15(fs::read_to_string("./inputs/2022/day15.txt").unwrap().as_str());
    day16::day16(fs::read_to_string("./inputs/2022/day16.txt").unwrap().as_str());
    day17::day17(fs::read_to_string("./inputs/2022/day17.txt").unwrap().as_str());
    day18::day18(fs::read_to_string("./inputs/2022/day18.txt").unwrap().as_str());
    day19::day19(fs::read_to_string("./inputs/2022/day19.txt").unwrap().as_str());
    day20::day20(fs::read_to_string("./inputs/2022/day20.txt").unwrap().as_str());
    day21::day21(fs::read_to_string("./inputs/2022/day21.txt").unwrap().as_str());
    day22::day22(fs::read_to_string("./inputs/2022/day22.txt").unwrap().as_str());
    day23::day23(fs::read_to_string("./inputs/2022/day23.txt").unwrap().as_str());
    day24::day24(fs::read_to_string("./inputs/2022/day24.txt").unwrap().as_str());
    day25::day25(fs::read_to_string("./inputs/2022/day25.txt").unwrap().as_str());
}