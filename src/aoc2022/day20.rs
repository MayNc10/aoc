use std::{fmt::{Display, Debug}, time::Instant};

#[derive(Clone, Copy)]
struct Coord {
    pub val: i128,
    pub original_position: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}
impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn make_coords(input: &str) -> Vec<Coord> {
    let mut coords = Vec::new();
    let mut pos = 0;
    for line in input.split("\n") {
        let val = line.parse().unwrap();
        coords.push(Coord {
            val,
            original_position: pos,
        });
        pos += 1;
    }
    coords
}

fn mix_coords(coords: &mut Vec<Coord>) {
    for pos in 0..coords.len() {
        let coord_idx = coords.iter().position(|coord| coord.original_position == pos).unwrap();

        let coord = coords[coord_idx];
        coords.remove(coord_idx);

        let mut new_loc = coord_idx as i128 + coord.val;
        if new_loc < 0 {
            new_loc = coords.len() as i128 + (new_loc % coords.len() as i128);
        }
        else {
            new_loc %= coords.len() as i128;
        }
        assert!(new_loc >= 0);
        coords.insert(new_loc as usize, coord);
    }
}

pub fn part1(input: &str) {
    let mut coords = make_coords(input);
    mix_coords(&mut coords);
    //println!("{:?}", coords);
    let idxs = [1000, 2000, 3000];
    let base = coords.iter().position(|coord| coord.val == 0).unwrap();
    let mut sum = 0;
    for val in idxs {
        let val = (val + base) % coords.len();
        sum += coords[val].val;
    }
    println!("{}", sum);
}

const KEY: i128 = 811589153;

pub fn part2(input: &str) {
    let mut coords = make_coords(input);
    for coord in coords.iter_mut() {
        coord.val *= KEY;
    }
    for _ in 0..10 { 
        mix_coords(&mut coords); 
        //println!("{:?}", coords);
    }
    
    let idxs = [1000, 2000, 3000];
    let base = coords.iter().position(|coord| coord.val == 0).unwrap();
    let mut sum = 0;
    for val in idxs {
        let val = (val + base) % coords.len();
        sum += coords[val].val;
    }
    println!("{}", sum);
}

pub fn day20(input: &str) {
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    println!("Completed day 20 part 1 in {:?}", after_p1.duration_since(now));
    let now = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Completed day 20 part 2 in {:?}", after_p2.duration_since(now));
}