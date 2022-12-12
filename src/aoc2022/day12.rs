use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos<'a> {
    pub coords: (i32, i32),
    pub board: &'a Vec<Vec<u8>>,
}

impl<'a> Pos<'a> {
    pub fn distance(&self, other: &Pos) -> usize {
        ((self.coords.0 - other.coords.0).abs() + (self.coords.1 - other.coords.1).abs()) as usize
    }
    pub fn neighbors(&self) -> Vec<(Pos<'a>, usize)> {
        let mut neighbors = Vec::new();
        for row_inc in -1..2 {
            let new_row = self.coords.0 + row_inc;
            if new_row < 0 || new_row as usize >= self.board.len() {
                continue;
            }
            for col_inc in -1..2 {
                if (row_inc == 0) == (col_inc == 0) { continue; }

                let new_col = self.coords.1 + col_inc;
                if new_col < 0 || new_col as usize >= self.board[0].len() {
                    continue;
                }
                let (new_row, new_col) = (new_row as usize, new_col as usize);
                let (row, col) = (self.coords.0 as usize, self.coords.1 as usize);
                if (self.board[new_row][new_col] as i32) - (self.board[row][col] as i32) < 2 {
                    neighbors.push((Pos {coords: (new_row as i32, new_col as i32), board: self.board}, 1))
                }
            }
        }
        neighbors
    }
}

pub fn parse_board(input: &str) -> (Vec<Vec<u8>>, (i32, i32), (i32, i32)) {
    let mut board = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut ridx = 0;
    for line in input.split("\n") {
        let mut row = Vec::new();
        let mut cidx = 0;
        for &b in line.as_bytes() {
            if b == 'S' as u8 {
                row.push(0);
                start = (ridx as i32, cidx as i32);
            }
            else if b == 'E' as u8 {
                row.push(25);
                end = (ridx as i32, cidx as i32);
            } 
            else {
                row.push(b - 97);
            }
            cidx += 1;
        }
        ridx += 1;
        board.push(row);
    }
    (board, start, end)
}

pub fn part1(input: &str) {
    let (board, start_coords, end_coords) = parse_board(input);
    //println!("{board:?}");
    let (start, end) = (Pos {coords: start_coords, board: &board}, Pos {coords: end_coords, board: &board});
    let result = astar(&start, |p| p.neighbors(), 
        |p| p.distance(&end), |p| *p == end).expect("Path not found");
    println!("{}", result.1);
}

pub fn part2(input: &str) {
    let (board, _, end_coords) = parse_board(input);
    let end = Pos {coords: end_coords, board: &board};
    let mut min_dis = usize::MAX;
    for row_idx in 0..board.len() {
        let row = &board[row_idx];
        for col_idx in 0..row.len() {
            let height = row[col_idx];
            if height == 0 {
                let start_coords = (row_idx as i32, col_idx as i32);
                let start = Pos {coords: start_coords, board: &board};
                let res = astar(&start, |p| p.neighbors(), 
                |p| p.distance(&end), |p| *p == end);
                if let Some(res) = res {
                    if res.1 < min_dis {
                        min_dis = res.1;
                    }
                }
            }
        }
    }
    println!("{min_dis}");
}

pub fn day12(input: &str) {
    part1(input);
    part2(input);
}