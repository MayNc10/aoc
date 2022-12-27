use std::time::Instant;
use colored::Colorize;

pub fn part1(input: &str) {
    println!("{}", input.split("\n").into_iter().map(|s| {
        let (first, second) = s.split_at(s.len() / 2);
        let mut both = 0;
        for b in first.as_bytes() {
            if second.as_bytes().contains(b) {
                both = *b as u32;
                break;
            }
        }
        if both >= 97 {
            both - 96
        } else {
            both - 38
        }
    }).sum::<u32>());
}

pub fn part2(input: &str) {
    let mut total = 0;
    let mut splits = input.split("\n").collect::<Vec<&str>>();
    while splits.len() > 0 {
        let first = splits[0];
        let second = splits[1];
        let third = splits[2];

        let mut both = 0;
        for b in first.as_bytes() {
            if second.as_bytes().contains(b) && third.as_bytes().contains(b) {
                both = *b as u32;
                break;
            }
        }
        if both == 0 {
            panic!("{first}\n{second}\n{third}");
        }

        if both >= 97 {
            total += both - 96;
        } else {
            total += both - 38;
        }
        splits.remove(0); splits.remove(0); splits.remove(0);
    }
    println!("{total}");
}

pub fn day3(input: &str) {
    println!("{}", "Day 3:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}