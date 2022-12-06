pub fn part1(input: &str) {
    for idx in 0..input.len() - 4 {
        let four = &input[idx..idx + 4];
        let mut stop = true;
        for idx2 in 0..4 {
            let c = &four[idx2..idx2 + 1];
            for idx3 in 0..4 {
                if idx3 == idx2 {
                    continue;
                }

                if &four[idx3..idx3 + 1] == c {
                    stop = false;
                    break;
                }
            }
            if !stop {
                break;
            }
        }
        if stop {
            println!("{}", idx + 4);
            return;
        }
    }
}

pub fn part2(input: &str) {
    for idx in 0..input.len() - 14 {
        let four = &input[idx..idx + 14];
        let mut stop = true;
        for idx2 in 0..14 {
            let c = &four[idx2..idx2 + 1];
            for idx3 in 0..14 {
                if idx3 == idx2 {
                    continue;
                }

                if &four[idx3..idx3 + 1] == c {
                    stop = false;
                    break;
                }
            }
            if !stop {
                break;
            }
        }
        if stop {
            println!("{}", idx + 14);
            return;
        }
    }
}

pub fn day6(input: &str) {
    part1(input);
    part2(input);
}