fn loop_lines<F>(input: &str, mut f: F)
where
    F: FnMut(i32, i32) -> bool,
{
    let mut x = 1;
    let mut cycle = 1;
    for line in input.split("\n") {
        if f(cycle, x) {
            break;
        }
        if line == "noop" {
            cycle += 1;
        }
        else {
            let add: i32 = line.split_ascii_whitespace().skip(1).next().unwrap().parse().unwrap();
            cycle += 1;
            if f(cycle, x) {
                break;
            }
            x += add;
            cycle += 1;
        }
    }
}

pub fn part1(input: &str) {
    let mut sum = 0;
    loop_lines(input, |cycle, x| {
        if cycle > 220 {
            return true;
        }
        else if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        } 
        false
    });
    println!("{sum}");
}

pub fn part2(input: &str) {
    loop_lines(input, |cycle, x| {
        let pos = cycle - 1;
        if (pos % 40 as i32).abs_diff(x) < 2 {
            print!("#");
        }
        else {
            print!(".");
        }
        if cycle % 40 == 0 {
            println!();
        }
        false
    });    
}

pub fn day10(input: &str) {
    part1(input);
    part2(input);
}