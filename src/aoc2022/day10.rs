pub fn part1(input: &str) {
    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;
    for line in input.split("\n") {
        if cycle > 220 {
            break;
        }
        else if (cycle - 20) % 40 == 0 {
            //println!("Cycle {}, adding {}", cycle, cycle * x);
            sum += cycle * x;
        } 
        if line == "noop" {
            cycle += 1;
        } else {
            let add: i32 = line.split_ascii_whitespace().skip(1).next().unwrap().parse().unwrap();
            cycle += 1;
            if cycle > 220 {
                break;
            }
            else if (cycle - 20) % 40 == 0 {
                sum += cycle * x;
            } 
            //println!("{line}, Adding {add}");
            x += add;
            cycle += 1;
        }
    }
    println!("{sum}");
}

pub fn part2(input: &str) {
    let mut x = 1;
    let mut pos = 0;
    for line in input.split("\n") {
        if (pos % 40 as i32).abs_diff(x) < 2 {
            print!("#");
        }
        else {
            print!(".");
        }
        if (pos + 1) % 40 == 0 {
            println!();
        }
        if line == "noop" {
            pos += 1;
        } else {
            let add: i32 = line.split_ascii_whitespace().skip(1).next().unwrap().parse().unwrap();
            pos += 1;
            if (pos % 40 as i32).abs_diff(x) < 2 {
                print!("#");
            }
            else {
                print!(".");
            }
            if (pos + 1) % 40 == 0 {
                println!();
            }
            //println!("{line}, Adding {add}");
            x += add;
            pos += 1;
        }
    }
    
}

pub fn day10(input: &str) {
    part1(input);
    part2(input);
}