use std::time::Instant;

pub fn part1(input: &str) {
    let mut score = 0;
    for s in input.split("\n") {
        let (theirs, yours) = ((s.as_bytes()[0] - 65) as u32, (s.as_bytes()[2] - 88) as u32);
        score += yours + 1; 
        if yours == theirs { score += 3; }
        else if yours == (theirs + 1) % 3 { score += 6 }
    }
    println!("{score}");
}

pub fn part2(input: &str) {
    let mut score = 0;
    for s in input.split("\n") {
        let (theirs, end) = ((s.as_bytes()[0] - 65) as i32, s.as_bytes()[2] as i32 - 89);
        let mut yours = (theirs + end) % 3;
        if yours == -1 { yours = 2}
        score += yours + 1; 
        if yours == theirs { score += 3; }
        else if yours == (theirs + 1) % 3 { score += 6 }
    }
    println!("{score}");
}

pub fn day2(input: &str) {
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    println!("Completed day 2 part 1 in {:?}", after_p1.duration_since(now));
    let now = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Completed day 2 part 2 in {:?}", after_p2.duration_since(now));
}