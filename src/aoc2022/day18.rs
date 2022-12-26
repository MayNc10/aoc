use std::collections::HashMap;
use std::time::Instant;

use petgraph::Graph;
use petgraph::algo::astar;

fn get_coords(input: &str) -> Vec<(usize, usize, usize)> {
    let mut coords = Vec::new();
    for line in input.split("\n") {
        let mut coord_iter = line.split(",");
        let x = coord_iter.next().unwrap().parse().unwrap();        
        let y = coord_iter.next().unwrap().parse().unwrap();    
        let z = coord_iter.next().unwrap().parse().unwrap();     
        coords.push((x, y, z));   
    }
    coords
}

fn get_max_dimensions(coords: &Vec<(usize, usize, usize)>) ->  (isize, isize, isize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for coord in coords {
        if coord.0 > max_x {
            max_x = coord.0;
        }
        if coord.1 > max_y {
            max_y = coord.1;
        }
        if coord.2 > max_z {
            max_z = coord.2
        }
    }
    (max_x as isize + 1, max_y as isize + 1, max_z as isize + 1)
}

fn surface_area(cubes: &Vec<Vec<Vec<bool>>>, maxes: (isize, isize, isize)) -> usize {
    let (max_x, max_y, max_z) = maxes;
    let mut uncovered_sides = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                if !cubes[x as usize][y as usize][z as usize] { continue; }

                for x_offset in  [-1, 1] {
                    let actual_x = x + x_offset;
                    if actual_x < 0 || actual_x >= max_x || cubes[actual_x as usize][y as usize][z as usize] == false {
                        uncovered_sides += 1;
                    }
                }
                for y_offset in  [-1, 1] {
                    let actual_y = y + y_offset;
                    if actual_y < 0 || actual_y >= max_y || cubes[x as usize][actual_y as usize][z as usize] == false {
                        uncovered_sides += 1;
                    }
                }
                for z_offset in  [-1, 1] {
                    let actual_z = z + z_offset;
                    if actual_z < 0 || actual_z >= max_z || cubes[x as usize][y as usize][actual_z as usize] == false {
                        uncovered_sides += 1;
                    }
                }
            }
        }
    }   
    uncovered_sides
}

pub fn part1(input: &str) {
    let coords = get_coords(input);
    let (max_x, max_y, max_z) = get_max_dimensions(&coords);
    let mut cubes = vec![vec![vec![false; max_z as usize]; max_y as usize]; max_x as usize];
    for (x, y, z) in coords {
        cubes[x][y][z] = true;
    }
    //println!("{:?}", cubes);
    let uncovered_sides = surface_area(&cubes, (max_x, max_y, max_z));

    println!("{}", uncovered_sides)
}

pub fn part2(input: &str) {
    let coords = get_coords(input);
    let (max_x, max_y, max_z) = get_max_dimensions(&coords);
    let mut cubes = vec![vec![vec![false; max_z as usize]; max_y as usize]; max_x as usize];
    for (x, y, z) in coords {
        cubes[x][y][z] = true;
    }
    // Create graph of all air nodes
    let mut air_graph = Graph::new();
    let external_node = air_graph.add_node(());
    let mut coords_to_node_map = HashMap::new();
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                let xu = x as usize;
                let yu = y as usize;
                let zu = z as usize;
                if cubes[xu][yu][zu] { continue; }
                let node  = air_graph.add_node(());
                // If node on outside, connect to extenal
                if x == 0 || x == max_x - 1 
                || y == 0 || y == max_y - 1 
                || z == 0 || z == max_z - 1 {
                    air_graph.add_edge(node, external_node, ());
                } 
                coords_to_node_map.insert((x, y, z), node);
            }
        }
    }

    // connect adjactent air nodes
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                if let Some(node) = coords_to_node_map.get(&(x, y, z)) {
                    let node = *node;
                    for x_offset in  [-1, 1] {
                        let actual_x = x + x_offset;
                        if let Some(other_node) = coords_to_node_map.get(&(actual_x, y, z)) {
                            let other_node = *other_node;
                            air_graph.add_edge(node, other_node, ());
                        }
                    }
                    for y_offset in  [-1, 1] {
                        let actual_y = y + y_offset;
                        if let Some(other_node) = coords_to_node_map.get(&(x, actual_y, z)) {
                            let other_node = *other_node;
                            air_graph.add_edge(node, other_node, ());
                        }
                    }
                    for z_offset in  [-1, 1] {
                        let actual_z = z + z_offset;
                        if let Some(other_node) = coords_to_node_map.get(&(x, y, actual_z)) {
                            let other_node = *other_node;
                            air_graph.add_edge(node, other_node, ());
                        }
                    }
                }
            }
        }
    }
    // Actually see if nodes are accesible
    let mut coords_to_mutate = Vec::new();
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                let xu = x as usize;
                let yu = y as usize;
                let zu = z as usize;
                if cubes[xu][yu][zu] { continue; }
                let node = *coords_to_node_map.get(&(x, y, z)).unwrap();
                let path = astar(&air_graph, node, 
                |f| f == external_node, |_| 0, |_| 0);
                if path.is_none() {
                    coords_to_mutate.push((xu, yu, zu));
                }
            }
        }
    }
    
    for (x, y, z) in coords_to_mutate {
        cubes[x][y][z] = true;
    }

    //println!("{:?}", cubes);
    let uncovered_sides = surface_area(&cubes, (max_x, max_y, max_z));

    println!("{}", uncovered_sides) 
}
pub fn day18(input: &str) {
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    println!("Completed day 18 part 1 in {:?}", after_p1.duration_since(now));
    let now = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Completed day 18 part 2 in {:?}", after_p2.duration_since(now));
}