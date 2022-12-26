use std::time::Instant;

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
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    println!("Completed day 15 part 1 in {:?}", after_p1.duration_since(now));
    let now = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Completed day 15 part 2 in {:?}", after_p2.duration_since(now));
}