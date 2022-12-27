use std::time::Instant;
use colored::Colorize;

pub fn part1(input: &str) {
    let span_len = 4;
    
    let mut idx = 0;
    let mut len = 1; 
    while len < span_len + 1 {
        let s = &input[idx..idx + len];
        let newc = &input[idx + len..idx + len + 1];

        let mut s_idx = 0;
        while (s_idx < s.len()) && &s[s_idx..s_idx + 1] != newc  {
            s_idx += 1;
        }
        if s_idx == s.len() {
            len += 1;
            if s_idx + 1 == span_len {
                println!("{}", idx + len);
                return;
            }
            continue;
        }
        idx += s_idx + 1;
        len -= s_idx;
    } 
}

pub fn part2(input: &str) {
    let span_len = 14;
    
    let mut idx = 0;
    let mut len = 1; 
    while len < span_len + 1 {
        let s = &input[idx..idx + len];
        let newc = &input[idx + len..idx + len + 1];

        let mut s_idx = 0;
        while (s_idx < s.len()) && &s[s_idx..s_idx + 1] != newc  {
            s_idx += 1;
        }
        if s_idx == s.len() {
            len += 1;
            if s_idx + 1 == span_len {
                println!("{}", idx + len);
                return;
            }
            continue;
        }
        idx += s_idx + 1;
        len -= s_idx;
    } 
}

pub fn day6(input: &str) {
    println!("{}", "Day 6:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}