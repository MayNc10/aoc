use crate::utils::*;
use std::time::Instant;
use colored::Colorize;

pub fn part1(input: &str) {
    let mut cals = split_by_big_gap(input);

    let cals = cals.iter_mut().map(|s| {
        let mut total = 0;
        for st in s.split_ascii_whitespace() {
            total += st.parse::<i32>().unwrap();
        }
        total
    });
    println!("{}", cals.max().unwrap());
}

pub fn part2(input: &str) {
    let mut cals = split_by_big_gap(input);

    let mut cals = cals.iter_mut().map(|s| {
        let mut total = 0;
        for st in s.split_ascii_whitespace() {
            total += st.parse::<i32>().unwrap();
        }
        total
    }).collect::<Vec<i32>>();
    let mut true_tot = 0;
    for _ in 0..3 {
        let max = cals.iter().max().unwrap();
        true_tot += max;
        cals.remove(cals.iter().position(|num| num == max).unwrap());
    }
    println!("{true_tot}");
}

pub fn day1(input: &str) {
    println!("{}", "Day 1:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}
