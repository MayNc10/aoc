use std::{ops::{Index, IndexMut}, time::Instant};

#[derive(Debug, Clone)]
struct Row {
    pub row: Vec<char>,
    begin: usize,
}

impl Row {
    fn coord_in_bounds(&self, coord: usize) -> bool {
        self.begin <= coord && self.begin + self.row.len() > coord
    }
}

impl Index<usize> for Row {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index >= self.begin);
        &self.row[index - self.begin]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index >= self.begin);
        &mut self.row[index - self.begin]
    }
}

fn read_rows(input: &str) -> Vec<Row> {
    let mut rows = Vec::new();
    for line in input.split("\n") {
        if line.trim().is_empty() { break; }
        let mut row = Vec::new();
        let mut begin = 0;
        for c in line.chars() {
            if c == ' ' {
                if !row.is_empty() { break; }
                else {
                    begin += 1;
                }
            }
            else {
                row.push(c);
            }
        }
        rows.push(Row { row, begin})
    }
    rows

}

#[derive(Clone, Copy, Debug)]
enum Move {
    Left,
    Right,
    Straight(usize),
}

fn parse_directions(mut directions: &str) -> Vec<Move> {
    let mut moves = Vec::new();
    let mut current_loc = 0;
    while current_loc < directions.len() {
        let current_char = directions[current_loc..current_loc + 1].chars().next().unwrap();
        match current_char {
            'L' => {
                if current_loc > 0 {
                    let num = directions[0..current_loc].parse().unwrap();
                    moves.push(Move::Straight(num));
                }
                moves.push(Move::Left);
                directions = &directions[current_loc + 1..];
                current_loc = 0;

            },
            'R' => {
                if current_loc > 0 {
                    let num = directions[0..current_loc].parse().unwrap();
                    moves.push(Move::Straight(num));
                }
                moves.push(Move::Right);
                directions = &directions[current_loc + 1..];
                current_loc = 0;
            },
            _ => {
                current_loc += 1;
            }
        }
    }
    
    if current_loc > 0 {
        let num = directions[0..current_loc].parse().unwrap();
        moves.push(Move::Straight(num));
    }
    moves
}

// Right, Down, Left, Up
fn right(pos: (usize, usize)) -> (isize, isize) { (pos.0 as isize, pos.1 as isize + 1) }
fn down(pos: (usize, usize)) -> (isize, isize) { (pos.0 as isize + 1, pos.1 as isize) }
fn left(pos: (usize, usize)) -> (isize, isize) { (pos.0 as isize, pos.1 as isize - 1) }
fn up(pos: (usize, usize)) -> (isize, isize) { (pos.0 as isize - 1, pos.1 as isize) }

static FACING_OFFSETS: &[fn((usize, usize)) -> (isize, isize); 4] = &[right, down, left, up];

static RIGHT: usize = 0; 
static DOWN: usize = 1;
static LEFT: usize = 2;
static UP: usize = 3;

fn find_next_col(col: isize, original: (usize, usize), map: &Vec<Row>, offset: isize) -> usize {
    if col < 0 || 
    !map[original.0].coord_in_bounds(col as usize)
    {
        let mut new_col = original.1 as isize;
        while new_col >= 0 &&
        map[original.0].coord_in_bounds(new_col as usize)
        { new_col += offset };
        new_col -= offset;
        new_col as usize
    } else { col as usize }
} 

fn find_next_row(row: isize, original: (usize, usize), map: &Vec<Row>, offset: isize) -> usize {
    if row < 0 || 
    (row as usize) >= map.len() ||
    !map[row as usize].coord_in_bounds(original.1)
    {
        let mut new_row = original.0 as isize;
        while new_row >= 0 && 
        (new_row as usize) < map.len() && 
        map[new_row as usize].coord_in_bounds(original.1) 
        { new_row += offset};
        new_row -= offset;
        new_row as usize
    } else { row as usize }
} 

fn debug_movement(pos: (usize, usize), map: &mut Vec<Row>, move_idx: usize) {    
    let new_c = match move_idx {
        0 => '>',
        1 => 'v',
        2 => '<',
        3 => '^',
        _ => unreachable!(),
    };
    
    map[pos.0][pos.1] = new_c;
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ConnectionType {
    Straight,
    Reversed
}

#[derive(Clone, Copy)]
struct Edge {
    span1: ((usize, usize), (usize, usize)),    
    span2: ((usize, usize), (usize, usize)),
    span1_entrance_direction: usize,
    span2_entrance_direction: usize,
    ty: ConnectionType
}

fn span_is_hori(span: ((usize, usize), (usize, usize))) -> bool {
    span.0.0 == span.1.0
}
fn span_is_vert(span: ((usize, usize), (usize, usize))) -> bool {
    span.0.1 == span.1.1
}


fn point_is_within_span(span: ((usize, usize), (usize, usize)), point: (usize, usize)) -> bool {
    if span_is_hori(span) {
        point.0 == span.0.0 && (point.1 >= span.0.1 && point.1 <= span.1.1)
    } 
    else if span_is_vert(span) {
        point.1 == span.0.1 && (point.0 >= span.0.0 && point.0 <= span.1.0)
    } else { panic!() }
}

impl Edge {
    fn try_map_point(&self, point: (usize, usize), is_vertical: bool) -> Option<((usize, usize), usize)> {
        let (this_edge, other_edge, direction) = 
        if point_is_within_span(self.span1, point) && (span_is_vert(self.span1) != !is_vertical) {
            (self.span1, self.span2, self.span2_entrance_direction)
        } 
        else if point_is_within_span(self.span2, point) && (span_is_vert(self.span2) != !is_vertical) {
            (self.span2, self.span1, self.span1_entrance_direction)
        } 
        else { return None; };

        let offset: usize = if span_is_hori(this_edge) {
            point.1 - this_edge.0.1
        } 
        else {
            point.0 - this_edge.0.0 
        };
        let real_point: (usize, usize) = if self.ty == ConnectionType::Straight {
            if span_is_hori(other_edge) {
                (other_edge.0.0, other_edge.0.1 + offset)
            }
            else {
                (other_edge.0.0 + offset, other_edge.0.1)
            }
        }
        else {
            if span_is_hori(other_edge) {
                (other_edge.0.0, other_edge.1.1 - offset)
            }
            else {
                (other_edge.1.0 - offset, other_edge.0.1)
            }
        };

        Some((real_point, direction))

    }
}

fn find_next_col_with_edges(col: isize, original: (usize, usize), edges: &Vec<Edge>, 
    map: &Vec<Row>, original_direction: usize) -> ((usize, usize), usize) 
{
    if col < 0 || 
    !map[original.0].coord_in_bounds(col as usize)
    {
        for edge in edges {
            if let Some((new_point, direction)) = edge.try_map_point(original, true) { 
                return (new_point, direction);
            }
        }
        println!("Didn't find an edge to map point!");
        println!("Original coords: {:?}, new column: {}", original, col);
        panic!();
    }
    else {
        ((original.0, col as usize), original_direction)
    }
} 

fn find_next_row_with_edges(row: isize, original: (usize, usize), edges: &Vec<Edge>, 
    map: &Vec<Row>, original_direction: usize) -> ((usize, usize), usize) 
{
    if row < 0 || 
    (row as usize) >= map.len() ||
    !map[row as usize].coord_in_bounds(original.1)
    {
        for edge in edges {
            if let Some((new_point, direction)) = edge.try_map_point(original, false) {
                return (new_point, direction);
            }
        }
        println!("Didn't find an edge to map point!");
        println!("Original coords: {:?}, new row: {}", original, row);
        panic!();
    }
    else {
        ((row as usize, original.1), original_direction)
    }
} 


pub fn part1(input: &str) {
    let map = read_rows(input);

    let mut movement_map = map.clone();

    let len = input.split("\n").count();
    let directions = input.split("\n").skip(len - 1).next().unwrap();
    let directions = parse_directions(directions);

    //println!("{:?}", directions);
    
    let mut position = (0, map[0].begin); // Row, col
    // Find actual position
    while map[0][position.1] == '#' {
        position.1 += 1;
    }

    let mut move_idx = 0;

    for mv in &directions {
        match mv {
            Move::Left => {
                debug_movement(position, &mut movement_map, move_idx);

                if move_idx == 0 { move_idx = FACING_OFFSETS.len(); }
                move_idx -= 1;
            },
            Move::Right => {
                debug_movement(position, &mut movement_map, move_idx);

                move_idx += 1;
                move_idx %= FACING_OFFSETS.len();
            },
            Move::Straight(amount) => {
                for _ in 0..*amount {
                    debug_movement(position, &mut movement_map, move_idx);

                    let next = {
                        if move_idx % 2 == 1 {
                            let (row, col) = (FACING_OFFSETS[move_idx])(position);
                            (find_next_row(row, position, &map, (move_idx as isize) - 2), col as usize)
                        } else {
                            let (row, col) = (FACING_OFFSETS[move_idx])(position);
                            (row as usize, find_next_col(col, position, &map, (move_idx as isize) - 1 ))
                        }
                    };
                    // If we've hit a wall, just stop
                    if map[next.0][next.1] == '#' {
                        break;
                    }
                    else {
                        position = next;
                    }
                }

            }
        }
    }
    
    let pwd = 1000 * (position.0 + 1) + 4 * (position.1 + 1) + move_idx;
    println!("{}", pwd);

}

// True layout
static CHUNK_SIZE: usize = 50;

static EDGE_A: Edge = Edge {
    span1: ((CHUNK_SIZE * 3 - 1, CHUNK_SIZE), (CHUNK_SIZE * 3 - 1, CHUNK_SIZE * 2 - 1)),
    span2: ((CHUNK_SIZE * 3 , CHUNK_SIZE - 1), (CHUNK_SIZE * 4 - 1, CHUNK_SIZE - 1)),
    span1_entrance_direction: UP,
    span2_entrance_direction: LEFT,
    ty: ConnectionType::Straight,
};
static EDGE_B: Edge = Edge {
    span1: ((CHUNK_SIZE, CHUNK_SIZE), (CHUNK_SIZE * 2 - 1, CHUNK_SIZE)),
    span2: ((CHUNK_SIZE * 2, 0), (CHUNK_SIZE * 2, CHUNK_SIZE - 1)),
    span1_entrance_direction: RIGHT,
    span2_entrance_direction: DOWN,
    ty: ConnectionType::Straight,
};
static EDGE_C: Edge = Edge {
    span1: ((0, CHUNK_SIZE), (CHUNK_SIZE - 1, CHUNK_SIZE)),
    span2: ((CHUNK_SIZE * 2, 0), (CHUNK_SIZE * 3 - 1, 0)),
    span1_entrance_direction: RIGHT,
    span2_entrance_direction: RIGHT,
    ty: ConnectionType::Reversed,
};
static EDGE_D: Edge = Edge {
    span1: ((0, CHUNK_SIZE), (0, CHUNK_SIZE * 2 - 1)),
    span2: ((CHUNK_SIZE * 3, 0), (CHUNK_SIZE * 4 - 1, 0)),
    span1_entrance_direction: DOWN,
    span2_entrance_direction: RIGHT,
    ty: ConnectionType::Straight,
};
static EDGE_E: Edge = Edge {
    span1: ((0, CHUNK_SIZE * 2), (0, CHUNK_SIZE * 3 - 1)),
    span2: ((CHUNK_SIZE * 4 - 1, 0), (CHUNK_SIZE * 4 - 1, CHUNK_SIZE - 1)),
    span1_entrance_direction: DOWN,
    span2_entrance_direction: UP,
    ty: ConnectionType::Straight
};
static EDGE_F: Edge = Edge {
    span1: ((CHUNK_SIZE - 1, CHUNK_SIZE * 2), (CHUNK_SIZE - 1, CHUNK_SIZE * 3 - 1)),
    span2: ((CHUNK_SIZE, CHUNK_SIZE * 2 - 1), (CHUNK_SIZE * 2 - 1, CHUNK_SIZE * 2 - 1)),
    span1_entrance_direction: UP,
    span2_entrance_direction: LEFT,
    ty: ConnectionType::Straight,
};
static EDGE_G: Edge = Edge {
    span1: ((0, CHUNK_SIZE * 3 - 1), (CHUNK_SIZE - 1, CHUNK_SIZE * 3 - 1)),
    span2: ((CHUNK_SIZE * 2, CHUNK_SIZE * 2 - 1), (CHUNK_SIZE * 3 - 1, CHUNK_SIZE * 2 - 1)),
    span1_entrance_direction: LEFT,
    span2_entrance_direction: LEFT,
    ty: ConnectionType::Reversed,
};


pub fn part2(input: &str) {
    let edges = vec![EDGE_A, EDGE_B, EDGE_C, EDGE_D, EDGE_E, EDGE_F, EDGE_G];

    let map = read_rows(input);

    let mut movement_map = map.clone();
    let mut path = Vec::new();

    let len = input.split("\n").count();
    let directions = input.split("\n").skip(len - 1).next().unwrap();
    let directions = parse_directions(directions);
    
    let mut position = (0, map[0].begin); // Row, col
    // Find actual position
    while map[0][position.1] == '#' {
        position.1 += 1;
    }

    let mut move_idx = 0;

    for mv in &directions {
        match mv {
            Move::Left => {
                debug_movement(position, &mut movement_map, move_idx);
                path.push((position, move_idx));

                if move_idx == 0 { move_idx = FACING_OFFSETS.len(); }
                move_idx -= 1;
            },
            Move::Right => {
                debug_movement(position, &mut movement_map, move_idx);
                path.push((position, move_idx));

                move_idx += 1;
                move_idx %= FACING_OFFSETS.len();
            },
            Move::Straight(amount) => {
                for _count in 0..*amount {
                    debug_movement(position, &mut movement_map, move_idx);
                    path.push((position, move_idx));

                    let (next, new_move_idx) = {
                        if move_idx % 2 == 1 {
                            let (row, _col) = (FACING_OFFSETS[move_idx])(position);
                            find_next_row_with_edges(row, position, &edges, &map, move_idx)
                        } else {
                            let (_row, col) = (FACING_OFFSETS[move_idx])(position);
                            find_next_col_with_edges(col, position, &edges, &map, move_idx)
                        }
                    };
                    // If we've hit a wall, just stop
                    if map[next.0][next.1] == '#' {
                        break;
                    }
                    else {
                        move_idx = new_move_idx;
                        position = next;
                    }
                }

            }
        }
    }
    
    debug_movement(position, &mut movement_map, move_idx);

    
    movement_map[position.0][position.1] = '*';

    let pwd = 1000 * (position.0 + 1) + 4 * (position.1 + 1) + move_idx;
    println!("{}", pwd);

}

pub fn day22(input: &str) {
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    println!("Completed day 22 part 1 in {:?}", after_p1.duration_since(now));
    let now = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Completed day 22 part 2 in {:?}", after_p2.duration_since(now));
}