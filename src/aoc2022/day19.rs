use std::{ops::{Sub, Index}, cmp::Ordering};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, PartialEq, Eq, Debug)]
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

#[derive(Clone, PartialEq, Eq, Debug)]
enum Action {
    Waited,
    Built(Robot),
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Simulation {
    pub blueprint: Blueprint,
    pub items: Cost,
    pub num_geodes: u32,
    pub production: [u8; 4],
    pub time: u8,
    pub route: Vec<Action>,
    pub stage: Stage,
}

const P1_TIME_LIMIT: u8 = 24;
const P2_TIME_LIMIT: u8 = 32;
//const ORE_CUTOFF: u8 = 10;
//const CLAY_CUTOFF: u8 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Stage {
    Ore,
    Clay,
    Last,
}

fn stage_to_robot(stage: Stage) -> Robot {
    match stage {
        Stage::Ore => Robot::Ore,
        Stage::Clay => Robot::Clay,
        Stage::Last => Robot::Obsidian,
    }
}

fn robot_to_stage(rb: Robot) -> Stage {
    match rb {
        Robot::Ore => Stage::Ore,
        Robot::Clay => Stage::Clay,
        Robot::Obsidian | Robot::Geode => Stage::Last,
    }
}


fn next_stage(stage: Stage) -> Stage {
    match stage {
        Stage::Ore => Stage::Clay,
        Stage::Clay => Stage::Last,
        Stage::Last => unreachable!(),
    }
}

impl Simulation {
    fn new(blueprint: Blueprint) -> Simulation {
        Simulation { blueprint, 
            items: Cost { num_ore: 0, num_clay: 0, num_obsidian: 0 }, 
            num_geodes: 0, 
            production: [1, 0, 0, 0], 
            time: 1,
            route: Vec::new(),
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

    fn robots_can_build(&self) -> Vec<Robot> {
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
  
    fn find_best(&mut self) {
        if self.time > P1_TIME_LIMIT { return; }
        if self.time == P1_TIME_LIMIT {
            self.mine();
            return;
        }

        let rbs_can_build = self.robots_can_build();
        let mut new_simulations = Vec::new();
        for rb in rbs_can_build {
            let mut new_sim = self.clone();
            new_sim.time += 1;
            let (rb, left) = new_sim.try_build_robot(rb).unwrap();
            new_sim.items = left;
            new_sim.production[rb as u8 as usize] += 1;
            new_sim.mine();
            new_simulations.push(new_sim);
        }
        let mut new_self = self.clone();
        new_self.mine();
        new_self.time += 1;
        new_simulations.push(new_self);
        new_simulations.par_iter_mut()
        .for_each(|sim| sim.find_best());


        //if self.time == 1 {
        //    println!("New sims:");
        //    for sim in &new_simulations {
        //        println!("{:#?}", sim);
        //    }
        //    println!("------------------");
        //}

        let best = new_simulations.into_iter().max().unwrap();
        *self = best;
    }
    fn find_limits(&self, rb: Robot) -> Vec<Robot> {
        let mut robots = vec![Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];
        robots = robots.into_iter().filter(|robot| (*robot as u8) < rb as u8).collect::<Vec<Robot>>();
        robots = robots.into_iter().filter(|r| {
            let idx = *r as u8;
            self.blueprint[rb][idx] > self.production[idx as usize]
        }).collect();

        robots.sort_by(|r1, r2| {
            let idx1 = *r1 as u8;
            let idx2 = *r2 as u8;

            if self.production[idx1 as usize] == 0 {
                Ordering::Less
            }
            else if self.production[idx2 as usize] == 0 {
                Ordering::Greater
            }
            else {
                
                let rate1 = self.blueprint[rb][idx1] / self.production[idx1 as usize];
                let rate2 = self.blueprint[rb][idx2] / self.production[idx2 as usize];
                rate2.cmp(&rate1)
            }
        });
        robots
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

    fn find_limit(&self, rb: Robot) -> Option<Robot> {
        let robots = self.find_limits(rb);
        if robots.len() == 0 { None }
        else { Some(*robots.first().unwrap()) }
    }
  
    fn is_robot_needed(&self, rb: Robot) -> bool {
        let robots = [Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode].iter();
        let robots = robots.filter(|r| **r != rb);
        for r in robots {
            if self.blueprint[*r][rb as u8] > self.production[rb as u8 as usize] { return true; }
        }
        false
    }

    fn find_ultimate_limit(&self, rb: Robot) -> Option<Robot> {
        let limit = self.find_limit(rb);
        if limit.is_none() { return None; }
        // See if we can make that robot
        let limit = limit.unwrap();
        if self.try_build_robot(limit).is_some() { Some(limit) }
        else { self.find_ultimate_limit(limit) }

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

    fn find_best_limiting_reactant(&mut self, time_limit: u8) {
        if self.time == time_limit {
            self.mine();
            self.route.push(Action::Waited);

            //if self.route[2] == Action::Built(Robot::Clay) &&
            //self.route[4] == Action::Built(Robot::Clay) &&
            //self.route[6] == Action::Built(Robot::Clay)
            //{
            //    let mut tick = 1;
            //    for action in &self.route {
            //        println!("{tick}: {:?}", action);
            //        tick += 1;
            //    }
            //    println!("{}", self.num_geodes);
            //    println!("---------------------");
            //}

            return;
        }    
        if let Some((rb, left)) = self.try_build_robot(Robot::Geode) {
            //println!("Building a geode robot");
            self.items = left;
            self.mine();
            self.production[rb as u8 as usize] += 1;
            self.route.push(Action::Built(Robot::Geode));
        }
        // /* 
        else {
            let robots_to_build = self.find_ultimate_limits();
            let limit = self.find_ultimate_limit(Robot::Geode);
            if limit.is_some() { assert!(robots_to_build.contains(&limit.unwrap())); }
                
            //println!("{:#?}", self.blueprint);
            //println!("{:?}", self.production);
            //println!("{:?}", robots_to_build);
            //println!("-------------------");

            if robots_to_build.is_empty() {
                self.mine();
                self.route.push(Action::Waited);
            } else {    
                // Split based on whether to build the robot or not
                /* 
                let mut clone = self.clone();
                let robot = stage_to_robot(self.stage);

                if robots_to_build.contains(&robot) 
                && let Some((rb, left)) = clone.try_build_robot(robot)
                {
                    clone.items = left;
                    clone.mine();
                    clone.production[rb as u8 as usize] += 1;
                    clone.time += 1;
                    clone.route.push(Action::Built(rb));
                    clone.find_best_limiting_reactant(time_limit);
                }

                
                let mut progess_sim = None;
                if self.stage != Stage::Last {
                    let mut progress_sim_inner = self.clone();
                    if let Some((rb, left)) = 
                        progress_sim_inner.try_build_robot(stage_to_robot(next_stage(self.stage))) 
                    {
                        progress_sim_inner.stage = next_stage(self.stage);
                        progress_sim_inner.items = left;
                        progress_sim_inner.mine();
                        progress_sim_inner.production[rb as u8 as usize] += 1;
                        progress_sim_inner.time += 1;
                        progress_sim_inner.route.push(Action::Built(rb));
                        progress_sim_inner.find_best_limiting_reactant(time_limit);
                    }
                    progess_sim = Some(progress_sim_inner);
                }

                let mut limit_sim: Option<Simulation> = None;
                let mut limit_sim_inner = self.clone();
                if limit.is_some() && 
                let Some((rb, left)) = limit_sim_inner.try_build_robot(limit.unwrap())
                {
                    limit_sim_inner.items = left;
                    limit_sim_inner.mine();
                    limit_sim_inner.production[rb as u8 as usize] += 1;
                    limit_sim_inner.time += 1;
                    limit_sim_inner.route.push(Action::Built(rb));
                    limit_sim_inner.find_best_limiting_reactant(time_limit);
                    limit_sim = Some(limit_sim_inner);
                }
                */

                let mut should_dbg = false;
                if self.time == 14 && self.route.as_slice() == &[
                    Action::Waited,
                    Action::Waited,
                    Action::Built(Robot::Clay),
                    Action::Waited,
                    Action::Built(Robot::Clay),
                    Action::Waited,
                    Action::Built(Robot::Clay),
                    Action::Waited,
                    Action::Waited,
                    Action::Waited,
                    Action::Built(Robot::Obsidian),
                    Action::Built(Robot::Clay),
                    Action::Waited,
                ] {
                    should_dbg = true;
                }

                let mut sims = Vec::new();
                for rb in &robots_to_build {
                    if true {
                        let mut clone = self.clone();
                        if let Some((rb, left)) = clone.try_build_robot(*rb)
                        {
                            clone.items = left;
                            clone.mine();
                            clone.stage = robot_to_stage(rb);
                            clone.production[rb as u8 as usize] += 1;
                            clone.time += 1;
                            clone.route.push(Action::Built(rb));
                            clone.find_best_limiting_reactant(time_limit);
                            sims.push(clone);
                        }
                    }
                }

                // Check if it's worth to wait
                //let mut robots_could_build: Vec<Robot> = vec![Robot::Ore, Robot::Clay, Robot::Obsidian, Robot::Geode];
                //let mut rob_idx = 0;
                //
                //while rob_idx < robots_could_build.len() {
                //    let robo = robots_could_build[rob_idx];
                //    if robots_to_build
                //}
                let mut robots_to_check = robots_to_build;
                robots_to_check.push(Robot::Geode);
                
                let mut should_pause = None;
                // FIXME: This is trying to test whether it's worth waiting
                // The way we should test this is to say 
                // "Can i not buy a certain robot AND at some point will I be able to"?
                // We then ask that for all 3 robots and if the answer to any one of those questions is yes
                // We branch and just wait.

                for rb in &robots_to_check {
                    //if should_dbg {
                    //    println!("{:?}", self.blueprint[*rb]);
                    //}
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
                
                //if should_dbg { 
                //    println!("Pruned waiting: {}", waste_to_wait);
                //    println!("{:?}", self.items);
                //}

                if (should_pause.is_some() && should_pause.unwrap() < (time_limit - self.time)) || sims.is_empty() {
                    self.mine();
                    self.route.push(Action::Waited);
                    self.time += 1;
                    self.find_best_limiting_reactant(time_limit);
                }

                /* 
                if clone.num_geodes > self.num_geodes 
                || ( clone.num_geodes == self.num_geodes 
                    && clone.items.num_obsidian >= self.items.num_obsidian 
                    && clone.items.num_clay >= self.items.num_clay
                    && clone.items.num_ore >= self.items.num_ore)
                { *self = clone; }
                
                
                if let Some(progress_sim) = progess_sim {
                    if progress_sim.num_geodes > self.num_geodes 
                    || ( progress_sim.num_geodes == self.num_geodes 
                        && progress_sim.items.num_obsidian >= self.items.num_obsidian 
                        && progress_sim.items.num_clay >= self.items.num_clay
                        && progress_sim.items.num_ore >= self.items.num_ore)
                    { *self = progress_sim; }
                }

                if let Some(limit_sim) = limit_sim {
                    if limit_sim.num_geodes > self.num_geodes 
                    || ( limit_sim.num_geodes == self.num_geodes 
                        && limit_sim.items.num_obsidian >= self.items.num_obsidian 
                        && limit_sim.items.num_clay >= self.items.num_clay
                        && limit_sim.items.num_ore >= self.items.num_ore)
                    { *self = limit_sim; }
                }
                */
                
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
        // */
        
         /* 
        else {
            let mut robots = self.find_ultimate_limits(Robot::Geode);
            
            if self.time <= ORE_CUTOFF {
                if !robots.contains(&Robot::Ore) {
                    robots.push(Robot::Ore);
                }
            }
            else {
                robots = robots.into_iter().filter(|rb| *rb != Robot::Ore).collect();
            }
            if self.time <= CLAY_CUTOFF {
                if !robots.contains(&Robot::Clay) {
                    robots.push(Robot::Clay);
                }
            }
            else {
                //robots = robots.into_iter().filter(|rb| *rb != Robot::Clay).collect();
            }
            robots = robots.into_iter().filter(|rb| self.cost(*rb) <= self.items).collect();

            if robots.is_empty() {
                self.mine();
                self.route.push(Action::Waited);
            } else {    
                let mut sims = vec![self.clone(); robots.len()];
                let mut rb_idx = 0;
                for clone in &mut sims {
                    let (rb, left) = clone.try_build_robot(robots[rb_idx]).unwrap();
                    clone.items = left;
                    clone.mine();
                    clone.production[rb as u8 as usize] += 1;
                    clone.time += 1;
                    clone.route.push(Action::Built(rb));
                    clone.find_best_limiting_reactant(time_limit);

                    rb_idx += 1;
                }



                self.mine();
                self.route.push(Action::Waited);
                self.time += 1;
                self.find_best_limiting_reactant(time_limit);
                let best = sims.into_iter().max_by(|s1, s2| {
                    (s1.num_geodes, s1.items.num_obsidian, s1.items.num_clay, s1.items.num_clay)
                    .cmp(
                        &(s2.num_geodes, s2.items.num_obsidian, s2.items.num_clay, s2.items.num_clay)
                    )
                }).unwrap();

                if best.num_geodes > self.num_geodes 
                || ( best.num_geodes == self.num_geodes 
                    && best.items.num_obsidian >= self.items.num_obsidian 
                    && best.items.num_clay >= self.items.num_clay
                    && best.items.num_ore >= self.items.num_ore)
                { *self = best; }
                
                return;
            }
        }
         */
        //println!("{:#?}", self);
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
        //println!("{:#?}", sim);
        //let mut tick = 1;
        //for action in sim.route {
        //    println!("{tick}: {:?}", action);
        //    tick += 1;
        //}
        //println!("---------------------");
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
        //println!("{:#?}", sims.last().unwrap());
    }
    let mut levels = 1;
    for sim in sims {
        levels *= sim.num_geodes;
        //println!("{:#?}", sim);
        //let mut tick = 1;
        //for action in sim.route {
        //    println!("{tick}: {:?}", action);
        //    tick += 1;
        //}
        //println!("---------------------");
    }
    println!("{}", levels);
}

pub fn day19(input: &str) { 
    part1(input);
    part2(input);
}