use crate::utils::*;

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
    part1(input);
    part2(input);
}
