use std::{ops::{Sub, Index}, cmp::Ordering, time::Instant};
use colored::Colorize;

use itertools::Itertools;
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Cost {
    pub num_ore: u8,    
    pub num_clay: u8,
    pub num_obsidian: u8,    
}

impl Index<u8> for Cost {
    type Output = u8;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.num_ore,
            1 => &self.num_clay,
            2 => &self.num_obsidian,
            _ => panic!("cost index out of bounds"),
        }
    }
}

impl Sub for Cost {
    type Output = Option<Cost>;

    fn sub(self, rhs: Self) -> Self::Output { 
        if self.num_ore >= rhs.num_ore
        && self.num_obsidian >= rhs.num_obsidian
        && self.num_clay >= rhs.num_clay 
        {
            Some(Cost { num_ore: self.num_ore - rhs.num_ore, 
                num_clay: self.num_clay - rhs.num_clay, 
            num_obsidian: self.num_obsidian - rhs.num_obsidian })
        }
        else { None }
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.num_ore == other.num_ore
        && self.num_clay == other.num_clay
        && self.num_obsidian == other.num_obsidian {
            Some(Ordering::Equal)
        }
        else if self.num_ore <= other.num_ore
        && self.num_clay <= other.num_clay
        && self.num_obsidian <= other.num_obsidian {
            Some(Ordering::Less)
        }
        else if self.num_ore >= other.num_ore
        && self.num_clay >= other.num_clay
        && self.num_obsidian >= other.num_obsidian {
            Some(Ordering::Greater)
        }
        else { None }

    }
}

impl Cost {
    fn vectorize(&self) -> Vec<u8> {
        vec![self.num_ore, self.num_clay, self.num_obsidian]
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
struct Blueprint {
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,        
    geode_robot: Cost, 
    number: u32,   
}

impl Index<Robot> for Blueprint {
    type Output = Cost;
    fn index(&self, index: Robot) -> &Self::Output {
        match index {
            Robot::Ore => &self.ore_robot,
            Robot::Clay => &self.clay_robot,
            Robot::Obsidian => &self.obsidian_robot,
            Robot::Geode => &self.geode_robot, 
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
enum Robot {
    Ore = 0,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy)]
struct Simulation {
    pub blueprint: Blueprint,
    pub items: Cost,
    pub num_geodes: u32,
    pub production: [u8; 4],
    pub time: u8,
    //pub route: Vec<Action>,
    pub stage: Stage,
}

const P1_TIME_LIMIT: u8 = 24;
const P2_TIME_LIMIT: u8 = 32;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Stage {
    Ore,
    Clay,
    Last,
}

fn robot_to_stage(rb: Robot) -> Stage {
    match rb {
        Robot::Ore => Stage::Ore,
        Robot::Clay => Stage::Clay,
        Robot::Obsidian | Robot::Geode => Stage::Last,
    }
}

impl Simulation {
    fn new(blueprint: Blueprint) -> Simulation {
        Simulation { blueprint, 
            items: Cost { num_ore: 0, num_clay: 0, num_obsidian: 0 }, 
            num_geodes: 0, 
            production: [1, 0, 0, 0], 
            time: 1,
            //route: Vec::new(),
            stage: Stage::Ore,
        }
    }

    fn cost(&self, robot: Robot) -> Cost {
        match robot {
            Robot::Ore => self.blueprint.ore_robot,
            Robot::Clay => self.blueprint.clay_robot,
            Robot::Obsidian => self.blueprint.obsidian_robot,
            Robot::Geode => self.blueprint.geode_robot,
        }
    }

    fn try_build_robot(&self, robot: Robot) -> Option<(Robot, Cost)> {
        if self.cost(robot) <= self.items {
            Some((robot, (self.items - self.cost(robot)).unwrap() ))
        } else { None }
    }

    fn _robots_can_build(&self) -> Vec<Robot> {
        let mut robots = Vec::new();
        if let Some((rb, _)) = self.try_build_robot(Robot::Ore) {
            robots.push(rb);
        }
        if let Some((rb, _)) = self.try_build_robot(Robot::Clay) {
            robots.push(rb);
        }
        if let Some((rb, _)) = self.try_build_robot(Robot::Obsidian) {
            robots.push(rb);
        }
        if let Some((rb, _)) = self.try_build_robot(Robot::Geode) {
            robots.push(rb);
        }
        robots

    }

    fn mine(&mut self) {
        self.items.num_ore += self.production[0];
        self.items.num_clay += self.production[1];
        self.items.num_obsidian += self.production[2];
        self.num_geodes += self.production[3] as u32; 
    }

    fn find_ultimate_limits(&self) -> Vec<Robot> {
        let mut limits = Vec::new();
        if self.production[0] < self.blueprint[Robot::Geode].num_ore { limits.push(Robot::Ore); }
        if self.production[2] < self.blueprint[Robot::Geode].num_obsidian { 
            limits.push(Robot::Obsidian);
            // If we need obsidian, we might also need clay
            if self.production[1] < self.blueprint[Robot::Obsidian].num_clay {
                limits.push(Robot::Clay);
            }
            // If we need obsidian or clay, we might still need ore
            if !limits.contains(&Robot::Ore) && (
                self.production[0] < self.blueprint[Robot::Obsidian].num_ore || (
                    limits.contains(&Robot::Clay) && 
                    self.production[0] < self.blueprint[Robot::Clay].num_ore
                )
            ) {
                limits.push(Robot::Ore);
            }
        }
        limits
    }

    fn should_wait_to_buy(&self, rb: Robot) -> Option<u8> {
        // if we can already build it then there's definitely no use to waiting.
        let items = self.items.vectorize();
        let cost = self.blueprint[rb].vectorize();
        
        let mut have_enough = true;
        let mut max_wait = 0.0;
        for cost_idx in 0..cost.len() {
            let needed = cost[cost_idx];
            let have = items[cost_idx];
            if needed <= have { continue; }
            have_enough = false;
            // Otherwise we need this resource
            // If our production is zero, we will never have enough, so we shouldn't wait
            if self.production[cost_idx] == 0 { return None; }
            let wait = (needed - have) as f32 / self.production[cost_idx] as f32;
            if wait > max_wait { max_wait = wait; }
        }
        if !have_enough {
            Some(max_wait.ceil() as u8)
        } else { None }

    }

    // FIXME: IF WE DECIDE TO WAIT ON A ROBOT, JUST SPAWN SIMULATIONS FOR EACH ROBOT WE COULD BUY AND TIMESKIP TO THEM  
    fn find_best_limiting_reactant(&mut self, time_limit: u8) {
        if self.time == time_limit {
            self.mine();
            //self.route.push(Action::Waited);
            return;
        }    
        if let Some((rb, left)) = self.try_build_robot(Robot::Geode) {
            self.items = left;
            self.mine();
            self.production[rb as u8 as usize] += 1;
            //self.route.push(Action::Built(Robot::Geode));
        }
        else {
            let robots_to_build = self.find_ultimate_limits();
            //let limit = self.find_ultimate_limit(Robot::Geode);
            //if limit.is_some() { assert!(robots_to_build.contains(&limit.unwrap())); }

            if robots_to_build.is_empty() {
                self.mine();
                //self.route.push(Action::Waited);
            } else {    
                let mut sims = Vec::new();
                for rb in &robots_to_build {
                    let mut clone = self.clone();
                    if let Some((rb, left)) = clone.try_build_robot(*rb)
                    {
                        clone.items = left;
                        clone.mine();
                        clone.stage = robot_to_stage(rb);
                        clone.production[rb as u8 as usize] += 1;
                        clone.time += 1;
                        //clone.route.push(Action::Built(rb));
                        clone.find_best_limiting_reactant(time_limit);
                        sims.push(clone);
                    }
                }

                let mut robots_to_check = robots_to_build.iter()
                    .filter(|rb| {
                        self.try_build_robot(**rb).is_none()
                    })
                    .map(|rb| *rb)
                    .collect_vec();
                robots_to_check.push(Robot::Geode);
                
                let mut should_pause = None;

                for rb in &robots_to_check {
                    let res = self.should_wait_to_buy(*rb);
                    if res.is_some() {
                        if should_pause.is_some() {
                            if should_pause.unwrap() > res.unwrap() {
                                should_pause = res;
                            }
                        }
                        else { should_pause = res; }
                    }
                    
                }

                if (should_pause.is_some() && should_pause.unwrap() < (time_limit - self.time)) || sims.is_empty() {
                    self.mine();
                    //self.route.push(Action::Waited);
                    self.time += 1;
                    self.find_best_limiting_reactant(time_limit);
                }

                for sim in sims {
                    if sim.num_geodes > self.num_geodes 
                    || ( sim.num_geodes == self.num_geodes 
                        && sim.items.num_obsidian >= self.items.num_obsidian 
                        && sim.items.num_clay >= self.items.num_clay
                        && sim.items.num_ore >= self.items.num_ore)
                    { *self = sim; }
                }
                return;
                
            }
        }
        self.time += 1;
        self.find_best_limiting_reactant(time_limit);
    }
}

impl PartialOrd for Simulation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Simulation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        assert!(self.time >= P1_TIME_LIMIT);
        assert!(other.time >= P1_TIME_LIMIT);
        self.num_geodes.cmp(&other.num_geodes)
    }
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let mut prints = Vec::new();
    for line in input.split("\n") {
        let it = line.split_ascii_whitespace();
        let mut it = it.skip(1);
        let num = it.next().unwrap();
        let num = num[0..num.len() - 1].parse().unwrap();
        let mut it = it.skip(4);
        let ore_cost = it.next().unwrap().parse().unwrap();
        let ore_robot = Cost { num_ore: ore_cost, num_clay: 0, num_obsidian: 0 };

        let mut it = it.skip(5);
        let clay_cost = it.next().unwrap().parse().unwrap();
        let clay_robot = Cost { num_ore: clay_cost, num_clay: 0, num_obsidian: 0 };

        let mut it = it.skip(5);
        let ob_cost_p1 = it.next().unwrap().parse().unwrap();
        let mut it = it.skip(2);
        let ob_cost_p2 = it.next().unwrap().parse().unwrap();
        let obsidian_robot = Cost { num_ore: ob_cost_p1, num_clay: ob_cost_p2, num_obsidian: 0 };

        let mut it = it.skip(5);
        let gd_cost_p1 = it.next().unwrap().parse().unwrap();
        let mut it = it.skip(2);
        let gd_cost_p2 = it.next().unwrap().parse().unwrap();
        let geode_robot = Cost { num_ore: gd_cost_p1, num_clay: 0, num_obsidian: gd_cost_p2 };

        prints.push(Blueprint { 
            ore_robot,
            clay_robot, 
            obsidian_robot, 
            geode_robot, 
            number: num,
        });
    }
    prints
}

pub fn part1(input: &str) {
    let prints = parse_blueprints(input);
    let mut sims = Vec::new();
    for print in prints {
        sims.push(Simulation::new(print));
        sims.last_mut().unwrap().find_best_limiting_reactant(P1_TIME_LIMIT);
    }
    let mut levels = 0;
    for sim in sims {
        levels += sim.num_geodes * sim.blueprint.number;
    }
    println!("{}", levels);
    
}

pub fn part2(input: &str) {
    let mut prints = parse_blueprints(input);
    prints.truncate(3);
    let mut sims = Vec::new();
    for print in prints {
        sims.push(Simulation::new(print));
        sims.last_mut().unwrap().find_best_limiting_reactant(P2_TIME_LIMIT);
    }
    let mut levels = 1;
    for sim in sims {
        levels *= sim.num_geodes;
    }
    println!("{}", levels);
}

pub fn day19(input: &str) { 
    println!("{}", "Day 19:".green());
    let now = Instant::now();
    part1(input);
    let after_p1 = Instant::now();
    let now_p1 = Instant::now();
    part2(input);
    let after_p2 = Instant::now();
    println!("Part 1 in {}", format!("{:?}", after_p1.duration_since(now)).green());
    println!("Part 2 in {}", format!("{:?}", after_p2.duration_since(now_p1)).green());
}