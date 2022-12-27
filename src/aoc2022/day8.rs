use std::time::Instant;
use colored::Colorize;

pub fn part1(input: &str) {
    let mut trees = Vec::new();
    for line in input.split("\n") {
        let mut new_row = Vec::new();
        for b in line.as_bytes() {
            new_row.push(*b - 48);
        }
        trees.push(new_row);
    }
    let mut are_visible = 0;
    for row in 1..trees.len() - 1 {
        for col in 1..trees.len() - 1 {
            let mut visible = true;
            for row_idx in 0..row {
                if trees[row_idx][col] >= trees[row][col] {
                    visible = false;
                }
            }
            if visible {
                are_visible += 1;
                continue;
            }

            let mut visible = true;
            for row_idx in row + 1..trees.len() {
                if trees[row_idx][col] >= trees[row][col] {
                    visible = false;
                }
            }
            if visible {
                are_visible += 1;
                continue;
            }

            let mut visible = true;
            for col_idx in 0..col {
                if trees[row][col_idx] >= trees[row][col] {
                    visible = false;
                }
            }
            if visible {
                are_visible += 1;
                continue;
            }

            let mut visible = true;
            for col_idx in col + 1..trees.len() {
                if trees[row][col_idx] >= trees[row][col] {
                    visible = false;
                }
            }
            if visible {
                are_visible += 1;
                continue;
            }
        }
    }
    are_visible += (trees.len() - 2) * 4 + 4;
    println!("{are_visible}");
}

pub fn part2(input: &str) {
    let mut trees = Vec::new();
    for line in input.split("\n") {
        let mut new_row = Vec::new();
        for b in line.as_bytes() {
            new_row.push(*b - 48);
        }
        trees.push(new_row);
    }
    let mut best_scenic_score = 0;
    for row in 1..trees.len() - 1 {
        for col in 1..trees.len() - 1 {
            let mut scenic_score = 1;
 
            let mut broke = false;
            for row_idx in (0..row).rev() {
                if trees[row_idx][col] >= trees[row][col] {
                    scenic_score *= row - row_idx;
                    broke = true;
                    break;
                }
            }
            if !broke {
                scenic_score *= row;
            }
            //println!("{scenic_score}");
            let mut broke = false;
            for row_idx in row + 1..trees.len() {
                if trees[row_idx][col] >= trees[row][col] {
                    scenic_score *= row_idx - row;
                    broke = true;
                    break;
                }
            }
            if !broke {
                scenic_score *= trees.len() - row - 1;
            }
            //println!("{scenic_score}");
            let mut broke = false;
            for col_idx in (0..col).rev() {
                if trees[row][col_idx] >= trees[row][col] {
                    scenic_score *= col - col_idx;
                    broke = true;
                    break;
                }
            }
            if !broke {
                scenic_score *= col;
            }
            //println!("{scenic_score}");
            let mut broke = false;
            for col_idx in col + 1..trees.len() {
                if trees[row][col_idx] >= trees[row][col] {
                    scenic_score *= col_idx - col;
                    broke = true;
                    break;
                }
            }
            if !broke {
                scenic_score *= trees.len() - col - 1;
            }
            //println!("{scenic_score}");
            //println!("{} {} {}", row, col, trees[row][col]);
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    println!("{best_scenic_score}");
}

pub fn day8(input: &str) {
    println!("{}", "Day 8".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}