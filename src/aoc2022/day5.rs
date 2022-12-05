pub fn part1(input: &str) {
    let num_stacks = 9;
    let start_height = 8;
    let indexes = [1, 5, 9, 13, 17, 21, 25, 29, 33];

    let mut stacks = vec![Vec::<&str>::new(); num_stacks];
    let init = &mut input.split("\n").collect::<Vec<&str>>()[..start_height];
    init.reverse();

    for line in init {
        for idx in indexes {
            if !(&line[idx..idx + 1].trim().is_empty()) {
                stacks[idx / 4].push(&line[idx..idx + 1]);
            }
        }
    }

    for struc in &input.split("\n").collect::<Vec<&str>>()[start_height + 2..] {
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
    let num_stacks = 9;
    let start_height = 8;
    let indexes = [1, 5, 9, 13, 17, 21, 25, 29, 33];

    let mut stacks = vec![Vec::<&str>::new(); num_stacks];
    let init = &mut input.split("\n").collect::<Vec<&str>>()[..start_height];
    init.reverse();

    for line in init {
        for idx in indexes {
            if !(&line[idx..idx + 1].trim().is_empty()) {
                stacks[idx / 4].push(&line[idx..idx + 1]);
            }
        }
    }

    for struc in &input.split("\n").collect::<Vec<&str>>()[start_height + 2..] {
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