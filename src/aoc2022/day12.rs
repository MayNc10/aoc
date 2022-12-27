use std::time::Instant;
use colored::Colorize;

use petgraph::{Graph, Directed, stable_graph::NodeIndex};
use petgraph::algo::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos<'a> {
    pub coords: (i32, i32),
    pub board: &'a Vec<Vec<u8>>,
}

impl<'a> Pos<'a> {
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

fn parse_board(input: &str) -> (Vec<Vec<u8>>, (i32, i32), (i32, i32)) {
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

fn board_to_graph(board: &Vec<Vec<u8>>, end_coords: (i32, i32)) -> (Graph<(), (), Directed>, NodeIndex) {
    let mut graph = Graph::new();
    let mut end_node = NodeIndex::from(0);
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let n = graph.add_node(());
            if (row as i32, col as i32) == end_coords {
                end_node = n;
            }
        }
    }
    
    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let pos = Pos {coords: (row as i32, col as i32), board: &board};
            let pnode = NodeIndex::from((row * board[0].len() + col) as u32);
            for (other_pos, _) in pos.neighbors() {
                let other_node = NodeIndex::from((other_pos.coords.0 * board[0].len() as i32 + other_pos.coords.1) as u32);
                graph.add_edge(pnode, other_node, ());
            }
        }
    }

    (graph, end_node)
}

pub fn part1(input: &str) {
    let (board, start_coords, end_coords) = parse_board(input);
    let (graph, end_node) = board_to_graph(&board, end_coords);
    let start_node = NodeIndex::from((start_coords.0 * board[0].len() as i32 + start_coords.1) as u32); 
    let (dis, _) =  astar(&graph, start_node, 
        |n| n == end_node, 
        |_| 1, 
        |n| end_node.index().abs_diff(n.index())).expect("Path not found");
    println!("{}", dis);
}

pub fn part2(input: &str) {
    let (board, _, end_coords) = parse_board(input);
    let (graph, end_node) = board_to_graph(&board, end_coords);
    let mut min_dis = usize::MAX;
    for row_idx in 0..board.len() {
        let row = &board[row_idx];
        for col_idx in 0..row.len() {
            let height = row[col_idx];
            if height == 0 {
                let start = NodeIndex::from((row_idx * row.len() + col_idx) as u32);
                let res = astar(&graph, start, 
                    |n| n == end_node, 
                    |_| 1, 
                    |n| end_node.index().abs_diff(n.index()));
                if let Some((dis, _)) = res && dis < min_dis {
                    min_dis = dis;
                }
            }
        }
    }
    
    println!("{min_dis}");
}

pub fn day12(input: &str) {
    println!("{}", "Day 12:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}