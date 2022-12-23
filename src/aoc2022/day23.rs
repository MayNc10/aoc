use std::{collections::{HashMap, HashSet}};

// Row, col
type Position = (isize, isize);

pub struct Elf {
    pos: Position,
    choice: Option<Position>,
}

impl Elf {
    fn new(pos: Position) -> Elf {
        Elf { pos, choice: None }
    }
    fn choose(&mut self, map: &mut HashMap<Position, bool>, directions: &Vec<fn(Position) -> [Position; 3]>) 
        -> Option<[Position; 3]> {
        let mut found_any = false;
        for direction in directions {
            for choice in (direction)(self.pos) {
                if !map.contains_key(&choice) {
                    map.insert(choice, false);
                }
                if map[&choice] { found_any = true; }
            }
        }
        if !found_any {
            self.choice = None;
            return None;
        }
        let mut final_direction = None;
        for direction in directions {
            final_direction = Some((direction)(self.pos));

            let mut is_possible = true;
            for choice in (direction)(self.pos) {
                if !map.contains_key(&choice) {
                    map.insert(choice, false);
                }
                if map[&choice] { is_possible = false; }
            }
            if is_possible {
                self.choice = Some((direction)(self.pos)[1]);
                break;
            }
        }
        final_direction
    }
}

fn north(pos: Position) -> [Position; 3] {
    [(pos.0 - 1, pos.1 - 1), (pos.0 - 1, pos.1), (pos.0 - 1, pos.1 + 1)]
}
fn south(pos: Position) -> [Position; 3] {
    [(pos.0 + 1, pos.1 - 1), (pos.0 + 1, pos.1), (pos.0 + 1, pos.1 + 1)]
}
fn west(pos: Position) -> [Position; 3] {
    [(pos.0 - 1, pos.1 - 1), (pos.0 , pos.1 - 1), (pos.0 + 1, pos.1 - 1)]
}
fn east(pos: Position) -> [Position; 3] {
    [(pos.0 - 1, pos.1 + 1), (pos.0, pos.1 + 1), (pos.0 + 1, pos.1 + 1)]
}

static DIRECTIONS: [fn(Position) -> [Position; 3]; 4] = [north, south, west, east];

fn read_map(input: &str) -> (HashMap<Position, bool>, Vec<Elf>, isize, isize) {
    let mut map = HashMap::new();
    let mut elves = Vec::new();
    let mut max_col = 0;
    let mut row = 0;
    for line in input.split("\n") {
        let mut col = 0;
        for c in line.chars() {
            map.insert((row, col), match c {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            });
            if c == '#' {
                elves.push(Elf::new((row, col)));
            }

            col += 1;
        }
        if col > max_col { max_col = col; }
        row += 1;
    }
    (map, elves, row, max_col)
}

fn print_map(map: &mut HashMap<Position, bool>, min: Position, max: Position) {
    for row in min.0..max.0 {
        for col in min.1..max.1 {
            if !map.contains_key(&(row, col)) {
                map.insert((row, col), false);
            }
            print!("{}", 
                if map[&(row, col)] { "#" } else { "." }
            ); 
        }
        println!();
    }
}

fn map_is_correct(map: &HashMap<Position, bool>, elves: &Vec<Elf>) -> bool {
    for elf in elves {
        if !map[&elf.pos] { return false; }
    }
    for key in map.keys() {
        if map[key] {
            let mut one_found = false;
            for elf in elves { 
                if elf.pos == *key { one_found = true; }
            }
            if !one_found { return false; }
        }
    }
    true
}

const NUM_P1_ROUNDS: usize = 10;

fn shrink_areas(map: &mut HashMap<Position, bool>, mins: Position, maxes: Position) -> (Position, Position) {
    let (minr, minc) = mins;
    let (maxr, maxc) = maxes;
    let mut new_minr = minr;
    'outer: loop {
        for col in minc..maxc {
            if !map.contains_key(&(new_minr, col)) {
                map.insert((new_minr, col), false);
            }

            if map[&(new_minr, col)] { break 'outer; }
        }
        new_minr += 1;
    }

    let mut new_maxr = maxr;
    'outer: loop {
        for col in minc..maxc {
            if !map.contains_key(&(new_maxr - 1, col)) {
                map.insert((new_maxr - 1, col), false);
            }

            if map[&(new_maxr - 1, col)] { break 'outer; }
        }
        new_maxr -= 1;
    }

    let mut new_minc = minc;
    'outer: loop {
        for row in minr..maxr {
            if !map.contains_key(&(row, new_minc)) {
                map.insert((row, new_minc), false);
            }

            if map[&(row, new_minc)] { break 'outer; }
        }
        new_minc += 1;
    }

    let mut new_maxc = maxc;
    'outer: loop {
        for row in minr..maxr {
            if !map.contains_key(&(row, new_maxc - 1)) {
                map.insert((row, new_maxc - 1), false);
            }

            if map[&(row, new_maxc - 1)] { break 'outer; }
        }
        new_maxc -= 1;
    }

    ((new_minr, new_minc), (new_maxr, new_maxc))
}

pub fn part1(input: &str) {
    let mut directions = Vec::from(DIRECTIONS);
    let (mut map, mut elves, mut max_row, mut max_col) = read_map(input);
    let mut min_row = 0; let mut min_col = 0;
        
    //print_map(&mut map, (min_row, min_col), (max_row, max_col));
    //println!();

    for _ in 0..NUM_P1_ROUNDS {
        assert!(map_is_correct(&map, &elves));

        let mut positions_chosen = HashSet::new();
        let mut duplicates = HashSet::new();
        let mut all_none = true;

        for elf in &mut elves {
            let added = elf.choose(&mut map, &directions);
            if let Some(added) = added {
                for (row, col) in added {
                    // Maxes go outside the actual boundary

                    if row < min_row { min_row = row; }
                    else if row >= max_row { max_row = row + 1; }  

                    if col < min_col { min_col = col; }
                    else if col >= max_col { max_col = col + 1; }  
                }
            }
            if let Some(pos) = elf.choice {
                all_none = false;
                if !positions_chosen.insert(pos) {
                    duplicates.insert(pos);
                }
            }
        }
        
        if all_none { break; }

        for elf in elves.iter_mut()
            .filter(|elf| elf.choice.is_some() && !duplicates.contains(&elf.choice.unwrap())) 
        {
            let new_pos = elf.choice.unwrap();
            map.insert(elf.pos, false);
            map.insert(new_pos, true);
            elf.pos = new_pos;
            elf.choice = None;
        }
        // Cycle directions
        let mut new_directions = directions.split_off(1);
        new_directions.append(&mut directions);
        directions = new_directions;
    }

    //print_map(&mut map, (min_row, min_col), (max_row, max_col));
    //println!();

    let (new_mins, new_maxes) = shrink_areas(&mut map, (min_row, min_col), (max_row, max_col));
    let mut empty = 0;
    for row in new_mins.0..new_maxes.0 {
        for col in new_mins.1..new_maxes.1 {
            if !map.contains_key(&(row, col)) {
                map.insert((row, col), false);
            }
            if !map[&(row, col)] { empty += 1; }
        }
    }

    println!("{}", empty);
}

pub fn part2(input: &str) {
    let mut directions = Vec::from(DIRECTIONS);
    let (mut map, mut elves, mut max_row, mut max_col) = read_map(input);
    let mut min_row = 0; let mut min_col = 0;
        
    let mut count = 1;
    loop {

        assert!(map_is_correct(&map, &elves));

        let mut positions_chosen = HashSet::new();
        let mut duplicates = HashSet::new();
        let mut all_none = true;

        for elf in &mut elves {
            let added = elf.choose(&mut map, &directions);
            if let Some(added) = added {
                for (row, col) in added {
                    // Maxes go outside the actual boundary

                    if row < min_row { min_row = row; }
                    else if row >= max_row { max_row = row + 1; }  

                    if col < min_col { min_col = col; }
                    else if col >= max_col { max_col = col + 1; }  
                }
            }
            if let Some(pos) = elf.choice {
                all_none = false;
                if !positions_chosen.insert(pos) {
                    duplicates.insert(pos);
                }
            }
        }

        if all_none ||
        elves.iter_mut()
        .filter(|elf| elf.choice.is_some() && !duplicates.contains(&elf.choice.unwrap())) 
        .count() == 0
        {
            println!("{}", count);  
            break;
        }

        for elf in elves.iter_mut()
            .filter(|elf| elf.choice.is_some() && !duplicates.contains(&elf.choice.unwrap())) 
        {
            let new_pos = elf.choice.unwrap();
            map.insert(elf.pos, false);
            map.insert(new_pos, true);
            elf.pos = new_pos;
            elf.choice = None;
        }
        for elf in &mut elves {
            elf.choice = None;
        }

        count += 1;
        let mut new_directions = directions.split_off(1);
        new_directions.append(&mut directions);
        directions = new_directions;
    }

}

pub fn day23(input: &str) {
    part1(input);
    part2(input);
}