use std::str::Split;

fn parse_stacks(input: &str) -> (Vec<Vec<&str>>, Split<&str>) {
    let num_stacks = input.split("\n").next().unwrap().len() / 4 + 1;
    let mut stacks = vec![Vec::<&str>::new(); num_stacks];

    let mut lines = input.split("\n");

    loop {
        let line = lines.next().unwrap();
        if line[0..1].trim().is_empty() && !line[1..2].trim().is_empty() {
            break;
        }
        for idx in 0..num_stacks {
            let real_idx = idx * 4 + 1;
            if !(&line[real_idx..real_idx + 1].trim().is_empty()) {
                stacks[idx].push(&line[real_idx..real_idx + 1]);
            }
        }
    }

    lines.next(); // Skip blank line

    for stack in &mut stacks {
        stack.reverse();
    }
    (stacks, lines)
}

pub fn part1(input: &str) {
    let (mut stacks, mut lines) = parse_stacks(input);

    for struc in &mut lines {
        let struc = struc.replace("move", "");
        let struc = struc.replace("from", "");
        let struc = struc.replace("to", "");

        let nums = struc.split_ascii_whitespace().collect::<Vec<&str>>();

        let count = nums[0].parse().unwrap();
        let src: usize = nums[1].parse::<usize>().unwrap() - 1;
        let dest: usize = nums[2].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let item = stacks[src].pop().unwrap();

            stacks[dest].push(item);
        }
    }
    for stack in &stacks {            
        print!("{}", stack.last().unwrap());
    }
    println!();

}

pub fn part2(input: &str) {
    let (mut stacks, mut lines) = parse_stacks(input);

    for struc in &mut lines {
        let struc = struc.replace("move", "");
        let struc = struc.replace("from", "");
        let struc = struc.replace("to", "");

        let nums = struc.split_ascii_whitespace().collect::<Vec<&str>>();

        let count = nums[0].parse().unwrap();
        let src: usize = nums[1].parse::<usize>().unwrap() - 1;
        let dest: usize = nums[2].parse::<usize>().unwrap() - 1;

        for num in 0..count {
            let item = stacks[src][stacks[src].len() - (count - num)];
            stacks[dest].push(item);
        }
        for _ in 0..count {
            stacks[src].pop();
        }

    }
    for stack in &stacks {            
        print!("{}", stack.last().unwrap());
    }
    println!();
}

pub fn day5(input: &str) {
    part1(input);
    part2(input);
}