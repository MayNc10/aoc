use std::collections::HashSet;
use itertools::*;

type Position = (usize, usize);

fn up(pos: Position, max_row: usize, max_col: usize) -> Position {
    if pos.0 - 1 == 0 { (max_row - 1, pos.1) }
    else { (pos.0 - 1, pos.1) }    
} 
fn down(pos: Position, max_row: usize, max_col: usize) -> Position {
    if pos.0 + 1 == max_row { (1, pos.1) }
    else { (pos.0 + 1, pos.1) }    
} 
fn left(pos: Position, max_row: usize, max_col: usize) -> Position {
    if pos.1 - 1 == 0 { (pos.0, max_col - 1) }
    else { (pos.0, pos.1 - 1) }    
} 
fn right(pos: Position, max_row: usize, max_col: usize) -> Position {
    if pos.1 + 1 == max_col { (pos.0, 1) }
    else { (pos.0, pos.1 + 1) }      
} 



static DIRECTIONS: [fn(Position, usize, usize) -> Position; 4] = [up, down, left, right];
static START_POS: Position = (0, 1);

#[derive(Clone, Copy, Debug)]
struct Blizzard {
    idx: usize,
    pos: Position,
}

impl Blizzard {
    fn next_pos(&self, max_row: usize, max_col: usize) -> Position {
        (DIRECTIONS[self.idx])(self.pos, max_row, max_col)
    }
}

fn create_board(input: &str) -> (Vec<Blizzard>, usize, usize) {
    let mut blizzards = Vec::new();
    let mut col = 0;
    let mut row = 0;
    for line in input.split("\n") {
        col = 0;
        for c in line.chars() {
            match c {
                '^' => blizzards.push(Blizzard { idx: 0, pos: (row, col) }),
                'v' => blizzards.push(Blizzard { idx: 1, pos: (row, col) }),
                '<' => blizzards.push(Blizzard { idx: 2, pos: (row, col) }),
                '>' => blizzards.push(Blizzard { idx: 3, pos: (row, col) }),
                _ => (),
            }
            col += 1;
        }
        row += 1;
    }
    (blizzards, row - 1, col - 1)
}

fn is_legal_pos(pos: Position, max_row: usize, max_col: usize) -> bool {
    if pos == START_POS { true }
    else if pos == (max_row, max_col - 1) { true }
    else { pos.0 != 0 && pos.0 != max_row && pos.1 != 0 && pos.1 != max_col }
}

// This could be made faster by not spliting, because the blizzards are the same for every sim
fn search(blizzards: Vec<Blizzard>, pos: Position, max_row: usize, max_col: usize, minute: usize) -> usize {
    let next = blizzards.iter()
    .map(|b| Blizzard { idx: b.idx, pos: b.next_pos(max_row, max_col) })
    .collect::<Vec<_>>();
    //println!("Blizzards: {:#?}", blizzards);
    //println!("Next Blizzards: {:#?}", next);

    let mut possibilities = Vec::new();
    // up
    if pos.0 != 0 {
        let new_pos = (pos.0 - 1, pos.1);
        if is_legal_pos(new_pos, max_row, max_col) { possibilities.push(new_pos); }
    }
    // down
    if pos.0 != max_row {
        let new_pos = (pos.0 + 1, pos.1);
        if is_legal_pos(new_pos, max_row, max_col) { possibilities.push(new_pos); }
    }
    // left
    if pos.1 != 0 {
        let new_pos = (pos.0, pos.1 - 1);
        if is_legal_pos(new_pos, max_row, max_col) { possibilities.push(new_pos); }
    }
    // right 
    if pos.1 != max_col {
        let new_pos = (pos.0, pos.1 + 1);
        if is_legal_pos(new_pos, max_row, max_col) { possibilities.push(new_pos); }
    }

    possibilities.push(pos);

    let mut idx = 0;
    while idx < possibilities.len() {
        let pos = possibilities[idx];
        let mut removed = false;
        for blizzard in &next {
            if blizzard.pos == pos {
                possibilities.remove(idx);
                removed = true;
                break;
            }
        }
        if !removed { idx += 1; }
    }

    println!("Pos: {:?}, Possibilities: {:?}", pos, possibilities);

    let mut min_time = usize::MAX;
    for pos in possibilities {
        if pos == (max_row, max_col - 1) { return minute; }
        let time = search(next.clone(), pos, max_row, max_col, minute + 1);
        if time < min_time {
            min_time = time;
        }
    }

    min_time
}

fn search_bfs(blizzards: Vec<Blizzard>, max_row: usize, max_col: usize) -> usize {
    let mut minute = 1;
    let mut next = blizzards.iter()
    .map(|b| Blizzard { idx: b.idx, pos: b.next_pos(max_row, max_col) })
    .collect::<Vec<_>>();
    let mut possible = vec![START_POS];
    loop {
        let mut new_possibilities = possible.clone();

        for pos in possible {
             // up
            if pos.0 != 0 {
                let new_pos = (pos.0 - 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // down
            if pos.0 != max_row {
                let new_pos = (pos.0 + 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // left
            if pos.1 != 0 {
                let new_pos = (pos.0, pos.1 - 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // right 
            if pos.1 != max_col {
                let new_pos = (pos.0, pos.1 + 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
        }
        new_possibilities.sort();
        new_possibilities.dedup();
        
        let mut idx = 0;
        while idx < new_possibilities.len() {
            let pos = new_possibilities[idx];
            let mut removed = false;
            for blizzard in &next {
                if blizzard.pos == pos {
                    new_possibilities.remove(idx);
                    removed = true;
                    break;
                }
            }
            if !removed { idx += 1; }
        }
        if new_possibilities.contains(&(max_row, max_col - 1)) { return minute; }
        possible = new_possibilities;

        minute += 1;
        for b in next.iter_mut() {
            b.pos = b.next_pos(max_row, max_col);
        }
    }
}

fn search_bfs_pt2(blizzards: Vec<Blizzard>, max_row: usize, max_col: usize) -> usize {
    let mut minute = 1;
    let mut next = blizzards.iter()
    .map(|b| Blizzard { idx: b.idx, pos: b.next_pos(max_row, max_col) })
    .collect::<Vec<_>>();
    let mut possible = vec![START_POS];
    'first: loop {
        let mut new_possibilities = possible.clone();

        for pos in possible {
             // up
            if pos.0 != 0 {
                let new_pos = (pos.0 - 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // down
            if pos.0 != max_row {
                let new_pos = (pos.0 + 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // left
            if pos.1 != 0 {
                let new_pos = (pos.0, pos.1 - 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // right 
            if pos.1 != max_col {
                let new_pos = (pos.0, pos.1 + 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
        }
        new_possibilities.sort();
        new_possibilities.dedup();
        
        let mut idx = 0;
        while idx < new_possibilities.len() {
            let pos = new_possibilities[idx];
            let mut removed = false;
            for blizzard in &next {
                if blizzard.pos == pos {
                    new_possibilities.remove(idx);
                    removed = true;
                    break;
                }
            }
            if !removed { idx += 1; }
        }
        if new_possibilities.contains(&(max_row, max_col - 1)) { break 'first;}
        possible = new_possibilities;

        minute += 1;
        for b in next.iter_mut() {
            b.pos = b.next_pos(max_row, max_col);
        }
    }
    minute += 1;
    for b in next.iter_mut() {
        b.pos = b.next_pos(max_row, max_col);
    }
    let mut possible = vec![(max_row, max_col - 1)];
    // Now go back
    'second: loop {
        let mut new_possibilities = possible.clone();

        for pos in possible {
             // up
            if pos.0 != 0 {
                let new_pos = (pos.0 - 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // down
            if pos.0 != max_row {
                let new_pos = (pos.0 + 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // left
            if pos.1 != 0 {
                let new_pos = (pos.0, pos.1 - 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // right 
            if pos.1 != max_col {
                let new_pos = (pos.0, pos.1 + 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
        }
        new_possibilities.sort();
        new_possibilities.dedup();
        
        let mut idx = 0;
        while idx < new_possibilities.len() {
            let pos = new_possibilities[idx];
            let mut removed = false;
            for blizzard in &next {
                if blizzard.pos == pos {
                    new_possibilities.remove(idx);
                    removed = true;
                    break;
                }
            }
            if !removed { idx += 1; }
        }
        if new_possibilities.contains(&(START_POS)) { break 'second;}
        possible = new_possibilities;

        minute += 1;
        for b in next.iter_mut() {
            b.pos = b.next_pos(max_row, max_col);
        }
    }
    // Now do it again
    minute += 1;
    for b in next.iter_mut() {
        b.pos = b.next_pos(max_row, max_col);
    }
    let mut possible = vec![START_POS];
    loop {
        let mut new_possibilities = possible.clone();

        for pos in possible {
             // up
            if pos.0 != 0 {
                let new_pos = (pos.0 - 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // down
            if pos.0 != max_row {
                let new_pos = (pos.0 + 1, pos.1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // left
            if pos.1 != 0 {
                let new_pos = (pos.0, pos.1 - 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
            // right 
            if pos.1 != max_col {
                let new_pos = (pos.0, pos.1 + 1);
                if is_legal_pos(new_pos, max_row, max_col) { new_possibilities.push(new_pos); }
            }
        }
        new_possibilities.sort();
        new_possibilities.dedup();
        
        let mut idx = 0;
        while idx < new_possibilities.len() {
            let pos = new_possibilities[idx];
            let mut removed = false;
            for blizzard in &next {
                if blizzard.pos == pos {
                    new_possibilities.remove(idx);
                    removed = true;
                    break;
                }
            }
            if !removed { idx += 1; }
        }
        if new_possibilities.contains(&(max_row, max_col - 1)) { return minute;}
        possible = new_possibilities;

        minute += 1;
        for b in next.iter_mut() {
            b.pos = b.next_pos(max_row, max_col);
        }
    }

}

fn print_valley(blizzards: &Vec<Blizzard>, max_row: usize, max_col: usize) {
    for row in 0..=max_row {
        for col in 0..=max_col {
            if (row, col) != START_POS && 
            (row, col) != (max_row, max_col - 1)
            && (row == 0 || row == max_row || col == 0 || col == max_col) {
                print!("#");
            }
            else {
                let mut found_count = 0;
                let mut found = None;
                for b in blizzards {
                    if b.pos == (row, col) { 
                        found_count += 1;
                        found = Some(*b);
                    }
                }
                if found_count == 0 {
                    print!(".");
                }
                else if found_count > 1 {
                    print!("{}", found_count);
                }
                else {
                    print!("{}", match found.unwrap().idx {
                        0 => "^",
                        1 => "v",
                        2 => "<",
                        3 => ">",
                        _ => unreachable!(),
                    });
                }
            }   
        }
        println!();
    }
}

pub fn part1(input: &str) {
    let (blizzards, max_row, max_col) = create_board(input);
    //print_valley(&blizzards, max_row, max_col);
    let min = search_bfs(blizzards.clone(), max_row, max_col);    
    println!("{}", min);
}

pub fn part2(input: &str) {
    let (blizzards, max_row, max_col) = create_board(input);
    //print_valley(&blizzards, max_row, max_col);
    let min = search_bfs_pt2(blizzards.clone(), max_row, max_col);    
    println!("{}", min);
}

pub fn day24(input: &str) {
    part1(input);
    part2(input);
}