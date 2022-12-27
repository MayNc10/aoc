use std::{collections::HashMap, time::Instant};
use colored::Colorize;

fn create_fs_layout(input: &str) -> HashMap<String, usize> {
    let mut dirs = HashMap::new();
    dirs.insert(String::from("/"), 0);
    let mut current_directory = String::from("/");
    for line in input.split("\n") {
        if &line[0..4] == "$ cd" {
            let changed_to = &line[5..line.len()];
            if changed_to == "/" {
                current_directory = String::from("/");
            } 
            else if changed_to == ".." {
                let last_dir_idx = current_directory.rfind("^").unwrap();
                current_directory.truncate(last_dir_idx);
            }
            else {
                current_directory.push('^');
                current_directory.push_str(&changed_to);
            }
        }
        else if &line[0..1] != "$" && &line[0..3] != "dir" {
            let mut temp_dir = current_directory.clone();
            let size: usize = line.split_ascii_whitespace().next().unwrap().parse().unwrap();
            while temp_dir != "/" {
                if !dirs.contains_key(&temp_dir) {
                    dirs.insert(temp_dir.clone(), size);
                } else {
                    let current_size = dirs.get_mut(&temp_dir).unwrap();
                    *current_size += size;
                }
                let last_dir_idx = temp_dir.rfind("^").unwrap();
                temp_dir.truncate(last_dir_idx);
            }
            let current_size = dirs.get_mut("/").unwrap();
            *current_size += size;
        }
    }
    dirs
}

pub fn part1(input: &str) {
    let dirs = create_fs_layout(input);
    let mut total = 0;
    for val in dirs.values() {
        if *val < 100000 {
            total += *val;
        }
    }
    println!("{total}");
}

pub fn part2(input: &str) {
    let dirs = create_fs_layout(input);
    let total_space_remaining = 70000000 - dirs.get("/").unwrap();
    let mut vals = dirs.values().collect::<Vec<&usize>>();
    vals.sort();
    for val in vals {
        if val + total_space_remaining > 30000000 {
            println!("{val}");
            return;
        }
    }
}

pub fn day7(input: &str) {
    println!("{}", "Day 7:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}