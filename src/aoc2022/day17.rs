use std::{collections::VecDeque, vec, time::Instant};
use colored::Colorize;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Movement {
    Left,
    Right
}

#[derive(Clone, Copy)]
enum Rock {
    Flat,
    Cross,
    L,
    Line,
    Square,
}

fn parse_movements(input: &str) -> VecDeque<Movement> {
    let mut movements= VecDeque::new();
    for b in input.as_bytes() {
        movements.push_back(match *b as char {
            '<' => Movement::Left,
            '>' => Movement::Right,
            _ => unreachable!(),
        });
    }
    movements
}

fn sim_movements(lines: &mut Vec<Vec<&str>>, movement: Movement) -> bool {
    // First, check if we can make the move 
    let mut can_make_movement = true;
    if movement == Movement::Left {
        for line_idx in 0..lines.len() {
            let line = &lines[line_idx];
            for shape_idx in 0..line.len() {
                let line = &lines[line_idx];
                if line[shape_idx] != "@" { continue; }
                if line_idx == 0 || lines[line_idx - 1][shape_idx] == "#" {
                    can_make_movement = false;
                }
            }
        } 
    }
    else {
        for line_idx in (0..lines.len()).rev() {
            let line = &lines[line_idx];
            for shape_idx in 0..line.len() {
                let line = &lines[line_idx];
                if line[shape_idx] != "@" { continue; }

                if line_idx == lines.len() - 1 || lines[line_idx + 1][shape_idx] == "#" {
                    can_make_movement = false;
                }
            }
            
        }
    }
    
    // Then, sim movements
    if can_make_movement {
        if movement == Movement::Left {
            for line_idx in 0..lines.len() {
                let line = &lines[line_idx];
                for shape_idx in 0..line.len() {
                    let line = &lines[line_idx];
                    if line[shape_idx] != "@" { continue; }
    
                    if line_idx == 0 {
                        continue;
                    }
                    if lines[line_idx - 1][shape_idx] != "." {
                        continue;
                    }
                    lines[line_idx - 1][shape_idx] = "@";
                    lines[line_idx][shape_idx] = ".";
                }
            } 
        }
        else {
            for line_idx in (0..lines.len()).rev() {
                let line = &lines[line_idx];
                for shape_idx in 0..line.len() {
                    let line = &lines[line_idx];
                    if line[shape_idx] != "@" { continue; }

                    if line_idx == lines.len() - 1 {
                        continue;
                    }
                    if lines[line_idx + 1][shape_idx] != "." {
                        continue;
                    }
                    lines[line_idx + 1][shape_idx] = "@";
                    lines[line_idx][shape_idx] = ".";
                }
                
            }
        }
    }

    // Then, check if we can move down
    let mut can_move_down = true;
    for line in &mut *lines {
        let shape_idx = line.iter().position(|&s| s == "@");
        if shape_idx.is_none() { continue; }
        let shape_idx = shape_idx.unwrap();
        if shape_idx == 0 || line[shape_idx - 1] != "." {
            can_move_down = false;
        }
    }
    if !can_move_down {
        for line in lines {
            for shape_idx in 0..line.len() {
                if line[shape_idx] != "@" { continue; }
                line[shape_idx] = "#";
            }
        }
        true
    }
    else {
        for line in lines {
            for shape_idx in 0..line.len() {
                if line[shape_idx] != "@" { continue; }
                line[shape_idx] = ".";
                line[shape_idx - 1] = "@";
            }
        }
        false
    }
    
}

fn spawn_rock(lines: &mut Vec<Vec<&str>>, rock: Rock) {
    // Add three new layers
    for line in &mut *lines {
        line.push(".");
        line.push(".");
        line.push(".");
    }
    match rock {
        Rock::Flat => {
            for line_idx in 0..7 {
                let line = &mut lines[line_idx];
                if line_idx > 1 && line_idx < 6 {
                    line.push("@");
                }
                else {
                    line.push(".");
                }
            }
        },
        Rock::Cross => {
            for line_idx in 0..7 {
                let line = &mut lines[line_idx];
                if line_idx == 2 || line_idx == 4 {
                    line.push(".");
                    line.push("@");
                    line.push(".");

                }
                else if line_idx == 3 {
                    line.push("@");
                    line.push("@");
                    line.push("@");
                }
                else {       
                    line.push(".");
                    line.push(".");
                    line.push(".");
                }
            }
        },
        Rock::L => {
            for line_idx in 0..7 {
                let line = &mut lines[line_idx];
                if line_idx == 2 || line_idx == 3 {
                    line.push("@");
                    line.push(".");
                    line.push(".");

                }
                else if line_idx == 4 {
                    line.push("@");
                    line.push("@");
                    line.push("@");
                }
                else {       
                    line.push(".");
                    line.push(".");
                    line.push(".");
                }
            }
        },
        Rock::Line => {
            for line_idx in 0..7 {
                let line = &mut lines[line_idx];
                if line_idx == 2 {
                    line.push("@");
                    line.push("@");
                    line.push("@");
                    line.push("@");
                }
                else {       
                    line.push(".");
                    line.push(".");
                    line.push(".");
                    line.push(".");
                }
            }

        },
        Rock::Square => {
            for line_idx in 0..7 {
                let line = &mut lines[line_idx];
                if line_idx == 2 || line_idx == 3 {
                    line.push("@");
                    line.push("@");

                }
                else {       
                    line.push(".");
                    line.push(".");
                }
            }
        },
    }
}

fn _print_lines(lines: &mut Vec<Vec<&str>>) {
    for idx in (0..lines[0].len()).rev() {
        for line in &*lines {
            print!("{}", line[idx]);
        }
        println!();
    }
}

fn _print_lines_with_limit(lines: &mut Vec<Vec<&str>>, limit: usize) {
    let cap = lines[0].len() - 1;
    for idx in 0..limit.min(cap + 1) {
        for line in &*lines {
            print!("{}", line[cap - idx]);
        }
        println!();
    }
}

fn clear_lines(lines: &mut Vec<Vec<&str>>) -> Option<usize> {
    // try to find an empty line
    let mut empty_idx = None;
    for idx in (0..lines[0].len()).rev() { 
        let mut line_clear = true;
        for line in &*lines {
            if line[idx] != "#" { line_clear = false }
        }
        if line_clear {
            empty_idx = Some(idx);
            break;
        }
    }
    if let Some(empty_idx) = empty_idx {
        for line_idx in 0..lines.len() {
            let new_line = lines[line_idx].split_off(empty_idx); 
            lines[line_idx] = new_line;
        }
    }

    empty_idx

}

const P1_NUM_LINES: i128 = 2022;

pub fn part1(input: &str) {
    let mut movements = parse_movements(input);
    let mut lines = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let mut rocks = VecDeque::from([Rock::Flat, Rock::Cross, Rock::L, Rock::Line, Rock::Square]);
    let mut fallen: i128 = 0;
    // Spawn first rock
    let rock = rocks.pop_front().unwrap();
    spawn_rock(&mut lines, rock);
    rocks.push_back(rock);
    while fallen < P1_NUM_LINES {

        let movement = movements.pop_front().unwrap();
        let res = sim_movements(&mut lines, movement);
        movements.push_back(movement);        
        // Skim top
        let mut top_empty = true;
        for line in &lines {
            if line.last().is_none() || *line.last().unwrap() != "." {
                top_empty = false;
            } 
        }
        if top_empty {
            for line in &mut lines {
                line.pop();
            }
        }

        if res {
            fallen += 1;
            // Don't create new rock if end
            if fallen >= P1_NUM_LINES {
                break;
            }
            let rock = rocks.pop_front().unwrap();
            spawn_rock(&mut lines, rock);
            rocks.push_back(rock);
        }
    }
    println!("{}", lines.iter().map(|v| v.len()).max().unwrap());
}

const P2_NUM_LINES: i128 = 1000000000000;

fn detect_cycle(cache: &Vec<(i128, usize, usize, u128)>) -> Option<(u128, usize)> {
    let start = cache.last()?;
    let mut first = None;
    for idx in (0..cache.len() - 1).rev() {
        let thing = cache[idx];
        if start.2 == thing.2 {
            first = Some(idx);
            break;
        }
    }
    let first = first?;
    let mut second = None;
    for idx in (0..first).rev() {
        let thing = cache[idx];
        if start.2 == thing.2 {
            second = Some(idx);
            break;
        }
    }
    let second = second?;
    let first_slice = &cache[first..cache.len() - 1];
    let second_slice = &cache[second..first];
    if first_slice.len() != second_slice.len() { return None; }
    for idx in 0..first_slice.len() {
        if first_slice[idx].2 != second_slice[idx].2 { return None; }
    }
    
    let first_diff = start.3 - cache[first].3;
    let second_diff = cache[first].3 - cache[second].3;
    if first_diff != second_diff { return None; }
    return Some((first_diff, (first - second) * 5));

} 

pub fn part2(input: &str) {
    let movements = parse_movements(input);
    let mut movement_idx = 0;
    let mut lines = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    let rocks = VecDeque::from([Rock::Flat, Rock::Cross, Rock::L, Rock::Line, Rock::Square]);
    let mut rock_idx = 0;
    let mut fallen: i128 = 0;
    //let mut map = HashMap::new();


    // Spawn first rock
    let rock = rocks[rock_idx];
    spawn_rock(&mut lines, rock);
    rock_idx += 1;
    rock_idx %= rocks.len();
    let mut cleared_lines: u128 = 0;
    let mut cache = Vec::new();


    while fallen < P2_NUM_LINES {
        let movement = movements[movement_idx];
        let res = sim_movements(&mut lines, movement);
        movement_idx += 1;
        movement_idx %= movements.len();
        // Skim top
        let mut top_empty = true;
        for line in &lines {
            if line.last().is_none() || *line.last().unwrap() != "." {
                top_empty = false;
            } 
        }
        if top_empty {
            for line in &mut lines {
                line.pop();
            }
        }

        let line_num = clear_lines(&mut lines);
        if let Some(line_num) = line_num {
            cleared_lines += line_num as u128;
        }

        if res {
            fallen += 1;
            if rock_idx == 0 {
                cache.push((fallen, rock_idx, movement_idx, 
                    lines.iter().map(|v| v.len()).max().unwrap() as u128 + cleared_lines));
                if let Some((fallen_in_cycle, rock_diff)) = detect_cycle(&cache) {
                    let num_left = (P2_NUM_LINES - fallen) as u128;
                    let num_cycles = num_left / rock_diff as u128;
                    cleared_lines += num_cycles * fallen_in_cycle;
                    fallen += (num_cycles * rock_diff as u128) as i128;
                    cache.clear();
                }
            }

            // Don't create new rock if end
            if fallen >= P2_NUM_LINES {
                break;
            }

            let rock = rocks[rock_idx];
            spawn_rock(&mut lines, rock);
            rock_idx += 1;
            rock_idx %= rocks.len();
        }

        
    }
    println!("{}", lines.iter().map(|v| v.len()).max().unwrap() as u128 + cleared_lines);
}

pub fn day17(input: &str) {
    println!("{}", "Day 17:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}