#[derive(PartialEq, Eq)]
enum FallResult {
    Moved,
    MovedAndStopped,
    FellIntoVoid
}

fn sim_sand_fall(map: &mut Vec<Vec<char>>, sand_idxs: (usize, usize)) -> (FallResult, Option<usize>) {
    let (row, col) = sand_idxs;
    // First, try to move down
    let new_row;
    if map[row].len() <= col + 1 {
        return (FallResult::FellIntoVoid, None);
    }
    if map[row][col + 1] == '.' {
        map[row][col] = '.';
        map[row][col + 1] = 'S';
        new_row = row;
    } 
    else {
        if row != 0 && map[row - 1].len() <= col + 1 {
            return (FallResult::FellIntoVoid, None);
        }
        if row != 0 && map[row - 1][col + 1] == '.' {
            map[row][col] = '.';
            map[row - 1][col + 1] = 'S';
            new_row = row - 1;
        }
        else {
            if map[row + 1].len() <= col + 1 {
                return (FallResult::FellIntoVoid, None);
            }
            map[row][col] = '.';
            map[row + 1][col + 1] = 'S';
            new_row = row + 1;
        }
    }
    
    if map[new_row].len() <= col + 2 {
        return (FallResult::FellIntoVoid, None);
    } 
    else if (map[new_row][col + 2] == '#' || map[new_row][col + 2] == 'O') {
        if new_row != 0 && map[new_row - 1].len() <= col + 2 {
            return (FallResult::FellIntoVoid, None);
        }
        else if new_row == 0 || (map[new_row - 1][col + 2] == '#' || map[new_row - 1][col + 2] == 'O') {
            if new_row < map.len() - 1 && map[new_row + 1].len() <= col + 2 {
                return (FallResult::FellIntoVoid, None);
            }
            else if new_row >= map.len() - 1 || (map[new_row + 1][col + 2] == '#' || map[new_row + 1][col + 2] == 'O')  {
                map[new_row][col + 1] = 'O';
                (FallResult::MovedAndStopped, Some(new_row))
            }
            else {
                (FallResult::Moved, Some(new_row))
            }
        }
        else {
            (FallResult::Moved, Some(new_row))
        }
    }
    else {
        (FallResult::Moved, Some(new_row))
    }
}

fn add_map_floor(map: &mut Vec<Vec<char>>) {
    let floor_col_idx = max_map_depth(map) + 1;
    for row_idx in 0..map.len() {
        ensure_coord_writable(map, row_idx, floor_col_idx);
        map[row_idx][floor_col_idx] = '#';
    }
}

fn ensure_coord_writable(map: &mut Vec<Vec<char>>, row: usize, col: usize) {
    if map.len() <= row {
        for _ in map.len()..row + 1 {
            map.push(Vec::new());
        }
    }
    if map[row].len() <= col {
        for _ in map[row].len()..col + 1 {
            map[row].push('.');
        }
    }
}

fn parse_line(line: &str, map: &mut Vec<Vec<char>>) {
    let coords = line.split(" -> ");
    let mut coords = coords.map(|coords| {
        let mut coords = coords.split(",");
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        (x, y)
    });
    let start = coords.next().unwrap();
    let mut last = start;
    for pair in coords {
        //println!("{:?}", pair);
        let (lastx, lasty) = last;
        let (x, y) = pair;
        if y == lasty {
            let mut range = lastx..x + 1;
            if x < lastx {
                range = x..lastx + 1;
            }

            for xcoord in range {
                ensure_coord_writable(map, xcoord, y);
                //println!("Writing to {},{}", xcoord, y);
                map[xcoord][y] = '#';
            }
        } else {
            let mut range = lasty..y + 1;
            if y < lasty {
                range = y..lasty + 1;
            }

            for ycoord in range {
                ensure_coord_writable(map, x, ycoord);
                //println!("Writing to {},{}", x, ycoord);
                map[x][ycoord] = '#';
            }
        }
        last = pair;
    } 
} 

fn max_map_depth(map: &Vec<Vec<char>>) -> usize {
    let mut max = 0;
    for row in map {
        if row.len() > max {
            max = row.len();
        }
    }
    max
}

fn print_map(map: &Vec<Vec<char>>) {
    for col_idx in 0..max_map_depth(map) {
        for row in map {
            // Check should skip
            let mut should_skip = true;
            for c in &row[0..row.len() - 1] {
                should_skip = should_skip && *c == '.';
            }

            if col_idx >= row.len() {
                continue;
            }
            if !should_skip {
                print!(" {} ", row[col_idx]);
            }
        }
        println!();
    }
}

pub fn part1(input: &str) {
    let mut map = Vec::new();
    for line in input.split("\n") {
        parse_line(line, &mut map);
    }
    ensure_coord_writable(&mut map, 500, 0);
    map[500][0] = '+';
    //print_map(&map);
    let mut to_rest = 0;
    let mut sand_idxs = (500, 1);
    loop {
        let (res, new_row) = sim_sand_fall(&mut map, sand_idxs);
        if res == FallResult::FellIntoVoid {
            break;
        }
        else if res == FallResult::MovedAndStopped {
            to_rest += 1;
            sand_idxs = (500, 1);
        }
        else {
            sand_idxs = (new_row.unwrap(), sand_idxs.1 + 1);
        }
    }
    println!("{}", to_rest);
}

pub fn part2(input: &str) {
    let mut map = Vec::new();
    for line in input.split("\n") {
        parse_line(line, &mut map);
    }
    // Continue floor rightwards
    let max_depth = max_map_depth(&map);
    for _ in 0..max_depth {
        let max_row = map.len();
        for col in 0..max_depth {
            ensure_coord_writable(&mut map, max_row, col);
        }
    }
    add_map_floor(&mut map);
    ensure_coord_writable(&mut map, 500, 0);
    map[500][0] = '+';
    let mut to_rest = 0;
    let mut sand_idxs = (500, 0);
    loop {
        let (res, new_row) = sim_sand_fall(&mut map, sand_idxs);
        if res == FallResult::MovedAndStopped {
            to_rest += 1;
            sand_idxs = (500, 0);
        }
        else {
            sand_idxs = (new_row.unwrap(), sand_idxs.1 + 1);
        }
        if map[500][0] == '.' {
            if map[500][1] == 'O' && map[499][1] == 'O' && map[501][1] == 'O' {
                break;
            }
            else {
                map[500][0] = '+';
            }
        } 
        
    }
    println!("{}", to_rest + 1);
   
}

pub fn day14(input: &str) {
    part1(input);
    part2(input);
}