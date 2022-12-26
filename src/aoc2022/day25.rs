// ty StackOverflow!
use std::{char::from_digit, time::Instant};
fn encode(mut n: u128, r: u128) -> Option<String> {
   let mut s = String::new();
   loop {
      if let Some(c) = from_digit((n % r)  as u32, r as u32) {
         s.insert(0, c)
      } else {
         return None
      }
      n /= r;
      if n == 0 {
         break
      }
   }
   Some(s)
}

fn snafu_to_decimal(snafu: &str) -> u128 {
    let mut val = 0;
    let mut place = 0;
    for c in snafu.chars().rev() {
        let place_val: i128 = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        };
        val += place_val * 5_i128.pow(place);
        place += 1; 
    }

    val as u128
}

fn decimal_to_snafu(dec: u128) -> String {
    let base_5 = encode(dec, 5).unwrap();
    let mut snafu = String::new();
    let mut carry = 0;
    for c in base_5.chars().rev() {
        let original_val = c as u8 - 48 + carry;
        let val = original_val % 5;
        carry /= 5;
        carry += original_val / 5;
        if val <= 2 { 
            snafu.insert(0, (val + 48) as char);
            continue;
        }
        // Means it a 3 or a 4
        carry += 1;
        if val == 3 {
            snafu.insert(0, '=');
        }
        else {
            assert_eq!(val, 4);
            snafu.insert(0, '-');
        }
    }
    if carry > 0 {

        if carry <= 2 { 
            snafu.insert(0, (carry + 48) as char);
        }
        // Means it a 3 or a 4
        else {
            if carry == 3 {
                snafu.insert(0, '=');
            }
            else {
                assert_eq!(carry, 4);
                snafu.insert(0, '-');
            }
            snafu.insert(0, '1');
        }
    }

    snafu
}

pub fn part1(input: &str) {
    let mut num = 0;
    for line in input.split("\n") {
        num += snafu_to_decimal(line);
    }
    println!("{}", decimal_to_snafu(num));
}

pub fn day25(input: &str) {
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    println!("Completed day 25 part 1 in {:?}", after_p1.duration_since(now));
}