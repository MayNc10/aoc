use std::{collections::HashSet, time::Instant};
use colored::Colorize;

fn adjust(head_pos: &mut (i32, i32), tail_pos: &mut(i32, i32)) {
    if (head_pos.0).abs_diff(tail_pos.0) < 2 && (head_pos.1).abs_diff(tail_pos.1) < 2 { return; }
    else  {
        if head_pos.0 > tail_pos.0 {
            tail_pos.0 += 1;
        } else if head_pos.0 < tail_pos.0 {
            tail_pos.0 -= 1;
        }
        if head_pos.1 > tail_pos.1 {
            tail_pos.1 += 1;
        } else if head_pos.1 < tail_pos.1 {
            tail_pos.1 -= 1;
        }
    }
}

fn sim_movement(head_pos: &mut (i32, i32), dir: &str) {
    match dir {
        "R" => head_pos.0 += 1,
        "L" => head_pos.0 -= 1,
        "U" => head_pos.1 += 1,
        "D" => head_pos.1 -= 1,
        _ => panic!("Had dir {}", dir),
    }
}

pub fn part1(input: &str) {
    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);
    let mut positions = HashSet::new();
    positions.insert(tail_pos);
    for line in input.split("\n") {
        let dir = &line[0..1];
        let amount = line[2..].parse().unwrap();
        for _ in 0..amount {
            sim_movement(&mut head_pos, dir);
            adjust(&mut head_pos, &mut tail_pos);
            positions.insert(tail_pos);
        }
    }
    
    println!("{}", positions.len());
}

pub fn part2(input: &str) {
    let mut knots = [(0,0); 10];
    let mut positions = HashSet::new();
    positions.insert(knots[9]);
    for line in input.split("\n") {
        let dir = &line[0..1];
        let amount = line[2..].parse().unwrap();
        for _ in 0..amount {
            sim_movement(&mut knots[0], dir);
            for idx in 0..9 {
                let (before, after) = knots.split_at_mut(idx + 1);
                adjust(&mut before[idx], &mut after[0]);
            }
            positions.insert(knots[9]);
        }
    }
    println!("{}", positions.len());
}

pub fn day9(input: &str) {
    println!("{}", "Day 9:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}