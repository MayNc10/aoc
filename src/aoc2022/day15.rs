use std::{collections::HashMap, hash::Hash, io};
use std::io::Write;
use rayon::prelude::*;

const COLUMN_AT: isize = 2000000;

#[repr(u8)]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Item {
    Empty,
    Sensor,
    Beacon,
    Unknown,
}

fn coords_to_idxs(coords: &str) -> (isize, isize) {
    let x = &coords[2..coords.find(",").unwrap()];
    let y = &coords[coords.rfind("=").unwrap() + 1..];
    (x.parse().unwrap(), y.parse().unwrap())
}

fn manhattan_distance(sensor: (isize, isize), beacon: (isize, isize)) -> usize {
    sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)
}

fn all_coords_with_mh_dis(sensor: (isize, isize), dis: usize) -> Vec<(isize, isize)> {
    //println!("Sensor at {sensor:?}, dis: {dis}");
    let dis = dis as isize;
    let mut coords = Vec::new();
    for x_offset in 0..dis + 1 {
        for y_offset in 0..(dis - x_offset) + 1 {
            coords.push((sensor.0 + x_offset, sensor.1 + y_offset));
            coords.push((sensor.0 + x_offset, sensor.1 - y_offset));
            coords.push((sensor.0 - x_offset, sensor.1 + y_offset));
            coords.push((sensor.0 - x_offset, sensor.1 - y_offset));
        }
    }
    //println!("Coords:");
    //for coord in &coords {
    //    println!("{coord:?}");
    //}
    //println!("-----------------------");
    coords
}

fn all_coords_with_mh_dis_add(sensor: (isize, isize), beacon: (isize, isize), 
    dis: usize, col: &mut Vec<Item>, min_x: isize, column_at: isize) {
    fn insert(col: &mut Vec<Item>, coord: (isize, isize), min_x: isize, 
        sensor: (isize, isize), beacon: (isize, isize), column_at: isize) {
        if coord.1 == column_at && coord.0 - min_x >= 0 && ((coord.0 - min_x) as usize) < col.len() {
            col[(coord.0 - min_x) as usize] = if coord == sensor {
                Item::Sensor
            }
            else if coord == beacon {
                Item::Beacon
            }
            else {
                Item::Empty
            }
        }
    }

    //println!("Sensor at {sensor:?}, dis: {dis}");
    let dis = dis as isize;
    for y_offset in 0..dis+1 {
        if sensor.1 + y_offset != column_at && sensor.1 - y_offset != column_at {
            continue;
        }

        for x_offset in 0..(dis - y_offset) + 1 {
            let coord = (sensor.0 + x_offset, sensor.1 + y_offset);
            insert(col, coord, min_x, sensor, beacon, column_at);
            
            let coord = (sensor.0 + x_offset, sensor.1 - y_offset);
            insert(col, coord, min_x, sensor, beacon, column_at);

            let coord = (sensor.0 - x_offset, sensor.1 + y_offset);
            insert(col, coord, min_x, sensor, beacon, column_at);

            let coord = (sensor.0 - x_offset, sensor.1 - y_offset);
            insert(col, coord, min_x, sensor, beacon, column_at);
        }
    }
}

fn all_coords_with_mh_dis_add_p2(sensor: (isize, isize), dis: usize, col: &mut Vec<Item>, min_x: isize, column_at: isize) {
    fn insert(col: &mut Vec<Item>, coord: (isize, isize), min_x: isize, 
        sensor: (isize, isize), column_at: isize) {
        if coord.1 == column_at && coord.0 - min_x >= 0 && ((coord.0 - min_x) as usize) < col.len() {
            col[(coord.0 - min_x) as usize] = Item::Empty;
        }
    }

    //println!("Sensor at {sensor:?}, dis: {dis}");
    let dis = dis as isize;
    for y_offset in 0..dis+1 {
        if sensor.1 + y_offset != column_at && sensor.1 - y_offset != column_at {
            continue;
        }

        for x_offset in 0..(dis - y_offset) + 1 {
            let coord = (sensor.0 + x_offset, sensor.1 + y_offset);
            insert(col, coord, min_x, sensor, column_at);
            
            let coord = (sensor.0 + x_offset, sensor.1 - y_offset);
            insert(col, coord, min_x, sensor, column_at);

            let coord = (sensor.0 - x_offset, sensor.1 + y_offset);
            insert(col, coord, min_x, sensor, column_at);

            let coord = (sensor.0 - x_offset, sensor.1 - y_offset);
            insert(col, coord, min_x, sensor, column_at);
        }
    }
}

fn do_mh_coords_intersect(sensor: (isize, isize),  dis: usize, intersect: (isize, isize))
    -> (bool, isize)
{
    let dis = dis as isize;
    for y_offset in 0..dis+1 {
        if sensor.1 + y_offset != intersect.1 && sensor.1 - y_offset != intersect.1 {
            continue;
        }

        for x_offset in (0..(dis - y_offset) + 1).rev() {
            let coord = (sensor.0 + x_offset, sensor.1 + y_offset);
            if coord == intersect {
                let hop_over = (x_offset + 1) * 2;
                return (true, hop_over);
            }
            
            let coord = (sensor.0 + x_offset, sensor.1 - y_offset);
            if coord == intersect {
                let hop_over = (x_offset + 1) * 2;
                return (true, hop_over);
            }

            let coord = (sensor.0 - x_offset, sensor.1 + y_offset);
            if coord == intersect {
                let hop_over = (x_offset + 1) * 2;
                return (true, hop_over);
            }

            let coord = (sensor.0 - x_offset, sensor.1 - y_offset);
            if coord == intersect {
                let hop_over = (x_offset + 1) * 2;
                return (true, hop_over);
            }
        }
    }
    (false, 0)
}

fn parse_map(input: &str) -> (HashMap<(isize, isize), Item>, (isize, isize), (isize, isize)) {
    let mut map = HashMap::new();
    let mut min_y = isize::MAX;
    let mut min_x = isize::MAX;
    let mut max_y = isize::MIN;
    let mut max_x = isize::MIN;
    for line in input.split("\n"){  
        let mut sensor_coords = &line[10..];
        sensor_coords = &sensor_coords[0..sensor_coords.find(":").unwrap()];
        let beacon_coords = &line[line.rfind("x=").unwrap()..];
        let sensor_coords = coords_to_idxs(sensor_coords);
        let beacon_coords = coords_to_idxs(beacon_coords);
        let dis = manhattan_distance(sensor_coords, beacon_coords) as isize;
        if sensor_coords.0 - dis < min_x {
            min_x = sensor_coords.0 - dis;
        }
        if sensor_coords.0 + dis > max_x {
            max_x = sensor_coords.0 + dis;
        }
        if sensor_coords.1 - dis < min_y {
            min_y = sensor_coords.1 - dis;
        }
        if sensor_coords.1 + dis > max_y {
            max_y = sensor_coords.1 + dis;
        }

    }
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            map.insert((x, y), Item::Unknown);
        }
    }

    for line in input.split("\n") {
        //print_map(&map, (min_x, min_y), (max_x, max_y));
        //println!("---------------------");
        let mut sensor_coords = &line[10..];
        sensor_coords = &sensor_coords[0..sensor_coords.find(":").unwrap()];
        let beacon_coords = &line[line.rfind("x=").unwrap()..];
        let sensor_coords = coords_to_idxs(sensor_coords);
        let beacon_coords = coords_to_idxs(beacon_coords);
        let coords = all_coords_with_mh_dis(sensor_coords, 
            manhattan_distance(sensor_coords, beacon_coords));
        for coord in coords {
            map.insert(coord, if coord == sensor_coords {
                Item::Sensor
           }
           else if coord == beacon_coords {
               Item::Beacon
           }
           else {
               Item::Empty
           });
        }

    }
    (map, (min_x, min_y), (max_x, max_y))
}
/* 
fn parse_map_with_bounds(input: &str, xs: (isize, isize), ys: (isize, isize)) -> HashMap<(isize, isize), Item> {
    let mut map = HashMap::new();
    for line in input.split("\n") {
        //print_map(&map, (min_x, min_y), (max_x, max_y));
        //println!("---------------------");
        let mut sensor_coords = &line[10..];
        sensor_coords = &sensor_coords[0..sensor_coords.find(":").unwrap()];
        let beacon_coords = &line[line.rfind("x=").unwrap()..];
        let sensor_coords = coords_to_idxs(sensor_coords);
        let beacon_coords = coords_to_idxs(beacon_coords);
        let coords = all_coords_with_mh_dis(sensor_coords, 
            manhattan_distance(sensor_coords, beacon_coords));
        for coord in coords {
            if 
            map.insert(coord, if coord == sensor_coords {
                Item::Sensor
            }
            else if coord == beacon_coords {
                Item::Beacon
            }
            else {
                Item::Empty
            });
        }

    }
    map
}
*/

fn print_map(map: &HashMap<(isize, isize), Item>, mins: (isize, isize), maxes: (isize, isize)) {
    for x in mins.0..maxes.0 + 1 {
        if x >= 0 && x < 10 {
            print!("{x}");
        }
        else {
            print!(".");
        }
    } 
    println!();
    for y in mins.1..maxes.1 + 1 {
        if y >= 0 && y < 10 {
            print!("{y} ");
        }
        else {
            print!(". ");
        }
        for x in mins.0..maxes.0 + 1 {
            let it = map[&(x, y)];
            print!("{}", match it {
                Item::Beacon => "B",
                Item::Empty => "#",
                Item::Sensor => "S",
                Item::Unknown => ".",
            });
        }
        println!();
    } 
}

pub fn part1(input: &str) {
    let mut min_y = isize::MAX;
    let mut min_x = isize::MAX;
    let mut max_y = isize::MIN;
    let mut max_x = isize::MIN;
    for line in input.split("\n"){  
        let mut sensor_coords = &line[10..];
        sensor_coords = &sensor_coords[0..sensor_coords.find(":").unwrap()];
        let beacon_coords = &line[line.rfind("x=").unwrap()..];
        let sensor_coords = coords_to_idxs(sensor_coords);
        let beacon_coords = coords_to_idxs(beacon_coords);
        let dis = manhattan_distance(sensor_coords, beacon_coords) as isize;
        if sensor_coords.0 - dis < min_x {
            min_x = sensor_coords.0 - dis;
        }
        if sensor_coords.0 + dis > max_x {
            max_x = sensor_coords.0 + dis;
        }
        if sensor_coords.1 - dis < min_y {
            min_y = sensor_coords.1 - dis;
        }
        if sensor_coords.1 + dis > max_y {
            max_y = sensor_coords.1 + dis;
        }

    }

    let mut col: Vec<Item> = vec![Item::Unknown; (max_x - min_x) as usize];
    
    for line in input.split("\n") {
        let mut sensor_coords = &line[10..];
        sensor_coords = &sensor_coords[0..sensor_coords.find(":").unwrap()];
        let beacon_coords = &line[line.rfind("x=").unwrap()..];
        let sensor_coords = coords_to_idxs(sensor_coords);
        let beacon_coords = coords_to_idxs(beacon_coords);
        
        all_coords_with_mh_dis_add(sensor_coords, beacon_coords,
            manhattan_distance(sensor_coords, beacon_coords),
            &mut col, min_x, COLUMN_AT);
    }
    

    let mut total = 0;
    for it in &col {
        if *it == Item::Empty {
            total += 1;
        }
    }

    println!("{total}");
    
}

const RANGE: isize = 4000000;
const CHUNK_SIZE: isize = 100;

pub fn part2(input: &str) {
    fn check_others(row: isize, col: isize, cache: &Vec<((isize, isize), usize)>, 
        sensor: &(isize, isize), dis: &usize) -> bool {
        for (other_sensor, other_dis) in cache {
            if other_sensor == sensor && other_dis == dis {
                continue;
            }
            else if manhattan_distance(*other_sensor, (row, col)) <= *other_dis {
                return false;
            }
        }
        true
    }
    fn coords_obt(coords: (isize, isize), maxes: (isize, isize), mins: (isize, isize)) -> bool {
        (coords.0 > maxes.0) || (coords.0 < mins.0) || (coords.1 > maxes.1) || (coords.1 < mins.1)
    }

    let mut cache = Vec::new();
    for line in input.split("\n") {
        let mut sensor_coords = &line[10..];
        sensor_coords = &sensor_coords[0..sensor_coords.find(":").unwrap()];
        let beacon_coords = &line[line.rfind("x=").unwrap()..];
        let sensor_coords = coords_to_idxs(sensor_coords);
        let beacon_coords = coords_to_idxs(beacon_coords);
        let dis = manhattan_distance(sensor_coords, beacon_coords);
        cache.push((sensor_coords, dis));
    }
    /* 
    (0..RANGE + 1).into_par_iter().for_each(|column_at| {
        //println!("{}", column_at);
        let mut col = vec![Item::Unknown; RANGE as usize];
        for (sensor, dis) in &cache {
            all_coords_with_mh_dis_add_p2(*sensor, *dis, &mut col, 0, column_at);
        }
        if let Some(row) = col.iter().position(|it| *it == Item::Unknown) {
            println!("{}", row as isize * 4000000 + column_at);
        }
    });
    */

    /* 
    let mut col = 0;
    while col <= RANGE {
        let mut row = 0;
        while row <= RANGE { 
            let mut early_broke = false;

            for line in input.split("\n") {
                let mut sensor_coords = &line[10..];
                sensor_coords = &sensor_coords[0..sensor_coords.find(":").unwrap()];
                let beacon_coords = &line[line.rfind("x=").unwrap()..];
                let sensor_coords = coords_to_idxs(sensor_coords);
                let beacon_coords = coords_to_idxs(beacon_coords);
                let (res, inc) = do_mh_coords_intersect(sensor_coords, beacon_coords, 
                    manhattan_distance(sensor_coords, beacon_coords),(row, col));
                
                if res
                {
                    early_broke = true;
                    row += inc;
                    break;
                }
            }
            if !early_broke {
                println!("{}", row * 4000000 + col);
                return;
            }
        }
        col += 1;
    }
    */
    let mut offset = 1;
    while offset < RANGE {
        for (sensor, dis) in &cache {
            let isize_dis = *dis as isize;
            for x in 0..(isize_dis + offset) {
                let y = (isize_dis + offset) - x;
                let mins = (0, 0);
                let maxes = (RANGE, RANGE);

                let coords = (sensor.0 + x, sensor.1 + y);
                if !coords_obt(coords, maxes, mins) {
                    let (row, col) = coords;
                    if check_others(row, col, &cache, sensor, dis) {
                        println!("{}, {}, {}", row, col,  row * 4000000 + col);
                        return;
                    }
                }
                let coords = (sensor.0 - x, sensor.1 + y);
                if !coords_obt(coords, maxes, mins) {
                    let (row, col) = coords;
                    if check_others(row, col, &cache, sensor, dis) {
                        println!("{}, {}, {}", row, col,  row * 4000000 + col);
                        return;
                    }
                }
                let coords = (sensor.0 + x, sensor.1 - y);
                if !coords_obt(coords, maxes, mins) {
                    let (row, col) = coords;
                    if check_others(row, col, &cache, sensor, dis) {
                        println!("{}, {}, {}", row, col,  row * 4000000 + col);
                        return;
                    }
                }
                let coords = (sensor.0 - x, sensor.1 - y);
                if !coords_obt(coords, maxes, mins) {
                    let (row, col) = coords;
                    if check_others(row, col, &cache, sensor, dis) {
                        println!("{}, {}, {}", row, col,  row * 4000000 + col);
                        return;
                    }
                }
            }
        }
        offset += 1;
    }
}

pub fn day15(input: &str) {
    part1(input);
    part2(input);
}