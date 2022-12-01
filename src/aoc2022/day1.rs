pub fn part1(input: &str) {
    let mut cals = input.split("\n    \n").collect::<Vec<&str>>();
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
    let mut cals = input.split("\n    \n").collect::<Vec<&str>>();
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
