pub fn part1(input: &str) {
    let mut total = 0;
    for line in input.split("\n") {

        let num1: i32 = line.split("-").next().unwrap().parse().unwrap();
        let num2: i32 = line.split(",").next().unwrap().split("-").skip(1).next().unwrap().parse().unwrap();
        let num3: i32 = line.split(",").skip(1).next().unwrap().split("-").next().unwrap().parse().unwrap();
        let num4: i32 = line.split("-").skip(2).next().unwrap().parse().unwrap();

        if (num1 >= num3 && num2 <= num4) || (num3 >= num1 && num4 <= num2)  {
            total += 1;
        }
    }
    println!("{total}");
}

pub fn part2(input: &str) {
    let mut total = 0;
    for line in input.split("\n") {

        let mut num1: i32 = line.split("-").next().unwrap().parse().unwrap();
        let mut num2: i32 = line.split(",").next().unwrap().split("-").skip(1).next().unwrap().parse().unwrap();
        let mut num3: i32 = line.split(",").skip(1).next().unwrap().split("-").next().unwrap().parse().unwrap();
        let mut num4: i32 = line.split("-").skip(2).next().unwrap().parse().unwrap();

        if num1 >= num3 {
            (num1, num3) = (num3, num1);
        }

        if num2 >= num4 {
            (num2, num4) = (num4, num2);
        }

        if num1 <= num4 && num3 <= num2  {
            total += 1;
        }
    }
    println!("{total}");
}

pub fn day4(input: &str) {
    part1(input);
    part2(input);
}