use std::time::Instant;

struct Monkey {
    pub items: Vec<u128>,
    pub change: Box<dyn Fn(&mut u128)>,
    pub test: u128,
    pub throw_idxs: (usize, usize),
}

impl Monkey {
    pub fn new(s: &str) -> Monkey {
        let it = s.split("\n");
        let mut it = it.skip(1);
        let mut starting = it.next().unwrap();
        starting = &starting[18..];
        let starting = starting.split(", ");
        let mut items = Vec::new();
        for line in starting {
            items.push(line.parse().unwrap());
        }
        let mut operation = it.next().unwrap();

        let divisible = it.next().unwrap();
        let divisible = divisible[21..].parse().unwrap();

        operation = &operation[23..];
        let change= if &operation[0..1] == "+" {
            if &operation[2..] == "old" {
                Box::new(|x: &mut u128| *x += *x) as Box<dyn Fn(&mut u128)>
            } else {
                let num: u128 = operation[2..].parse().unwrap();
                Box::new(move |x: &mut u128| *x += num) as Box<dyn Fn(&mut u128)>
            }
        } else {
            if &operation[2..] == "old" {
                Box::new(|x: &mut u128| *x *= *x) as Box<dyn Fn(&mut u128)>
            } else {
                let num: u128 = operation[2..].parse().unwrap();
                Box::new(move |x: &mut u128| *x *= num) as Box<dyn Fn(&mut u128)>
            }
        };
        
        let on_true = it.next().unwrap();
        let on_true = on_true[29..].parse().unwrap();
        let on_false = it.next().unwrap();
        let on_false = on_false[30..].parse().unwrap();

        Monkey { items, change,test: divisible, throw_idxs: (on_true, on_false) }
    }
}

fn parse_lines(input: &str) -> Vec<String> {
    let mut v = Vec::new();
    let mut s = String::new();
    for line in input.split("\n") {
        if line.trim().len() == 0 {
            v.push(s);
            s = String::new();
        }
        else {
            s.push_str(line);
            s.push_str("\n");
        }
    }
    v.push(s);
    v
}

pub fn part1(input: &str) {
    let mut monkeys = Vec::new();
    let mut max_possible = 1;
    for monkey in parse_lines(input) {
        monkeys.push(Monkey::new(monkey.as_str()));
        max_possible *= monkeys.last().unwrap().test;
    }
    
    let mut inspects = vec![0; monkeys.len()];
    
    
    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            while monkeys[monkey_idx].items.len() > 0 {
                let monkey = &mut monkeys[monkey_idx];
                let divis = monkey.test;
                let (true_idx, false_idx) = monkey.throw_idxs;
                let mut item = monkey.items.pop().unwrap();
                (monkey.change)(&mut item);
                inspects[monkey_idx] += 1;
                item /= 3;
                item %= max_possible;

                if item % divis == 0 {
                    monkeys[true_idx].items.push(item);
                } else {
                    monkeys[false_idx].items.push(item);
                }
                
            }
        }
    }

    inspects.sort();
    inspects.reverse();
    println!("{}", inspects[0] * inspects[1]);
}

pub fn part2(input: &str) {
    let mut monkeys = Vec::new();
    let mut max_possible = 1;
    for monkey in parse_lines(input) {
        monkeys.push(Monkey::new(monkey.as_str()));
        max_possible *= monkeys.last().unwrap().test;
    }

    let mut inspects = vec![0_u128; monkeys.len()];
    for _round in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            while monkeys[monkey_idx].items.len() > 0 {
                let monkey = &mut monkeys[monkey_idx];
                let divis = monkey.test;
                let (true_idx, false_idx) = monkey.throw_idxs;
                let mut item = monkey.items.pop().unwrap();

                (monkey.change)(&mut item);
                inspects[monkey_idx] += 1;

                item %= max_possible;
                if item % divis == 0_u128 {
                    monkeys[true_idx].items.push(item);
                } else {
                    monkeys[false_idx].items.push(item);
                }
                
            }
        }
    }
    inspects.sort();
    inspects.reverse();
    println!("{}", inspects[0] * inspects[1]);
}

pub fn day11(input: &str) {
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    println!("Completed day 11 part 1 in {:?}", after_p1.duration_since(now));
    let now = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Completed day 11 part 2 in {:?}", after_p2.duration_since(now));
}