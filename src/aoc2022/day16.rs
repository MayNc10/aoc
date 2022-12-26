use std::cmp::Ordering;
use std::collections::{VecDeque, HashMap, HashSet};
use std::hash::BuildHasherDefault;
use std::ops::Range;
use itertools::Itertools;

use petgraph::algo::{astar, floyd_warshall};
use petgraph::graph::NodeWeightsMut;

use petgraph::stable_graph::NodeIndex;
use petgraph::{Graph, Directed};
use petgraph::algo::dijkstra;
use petgraph::dot::{Dot, Config};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Valve<'a> {
    pub flow_rate: u32, 
    pub name: &'a str,
    pub connections: Vec<&'a str>,
    pub is_on: bool,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[repr(transparent)]
struct ValveState {
    pub state: u128
}
impl ValveState {
    pub fn new(valve_graph: &mut Graph<Valve, (), Directed>) -> ValveState {
        assert!(valve_graph.node_count() <= 128);
        let mut num = 0;
        let mut place = 1;
        for valve in valve_graph.node_weights() {
            if valve.is_on {
                num &= place;
            }
            place <<= 1; 
        }
        ValveState { state: num }
    }
}

fn make_valve<'a>(line: &'a str) -> Valve<'a> {
    let tokens = line.split_ascii_whitespace();
    let mut tokens = tokens.skip(1);
    let name = tokens.next().unwrap();
    let mut tokens = tokens.skip(2);
    let mut rate = tokens.next().unwrap();
    rate = &rate[5..];
    rate = &rate[..rate.len() - 1];
    let flow_rate = rate.parse().unwrap();
    let tokens = tokens.skip(4);
    let mut connections = Vec::new();
    let mut tokens: VecDeque<&str> = tokens.collect();
    while tokens.len() > 0 {
        let item = tokens.pop_front().unwrap();
        if tokens.len() == 0 {
            connections.push(item);
        } else {
            let len = item.len();
            connections.push(&item[0..len - 1]);
        }
    }
    Valve { flow_rate, name, connections, is_on: false}
}

fn score_valves_on_tick(valves: NodeWeightsMut<Valve>) -> u32 {
    valves.fold(0, |acc, v| 
        acc + if v.is_on {
            v.flow_rate
        } else { 0 }
    )
}

/* 
fn sim_step(valves: &mut HashMap<&str, Valve>, tick: u8, current_location: &str) -> u32 {
    if tick > 30 {
        return 0;
    }

    let current_score = score_valves_on_tick(valves);
    let current_valve = &valves[current_location];
    let mut max = if !current_valve.is_on {
        // If we can turn this valve on, do so
        valves.get_mut(current_location).unwrap().is_on = true;
        let score = sim_step(valves, tick + 1, current_location);
        // Reset valves
        valves.get_mut(current_location).unwrap().is_on = false;
        score 
    } else { 0 };
    let current_valve = &valves[current_location];
    for dest in (&current_valve.connections).clone() {
        let score = sim_step(valves, tick + 1, dest);
        if score > max {
            max = score;
        }
    }
    current_score + max
}
*/

fn branch(valve_graph: &mut Graph<Valve, (), Directed>, tick: u8, current_location: NodeIndex, max: u8, distances: &HashMap<(NodeIndex, NodeIndex), i32>) 
    -> (u32, Vec<(NodeIndex, u8)>) {
    //println!("{}", tick);
    
    if tick > max {
        return (0, Vec::new());
    }

    let base_score = score_valves_on_tick(valve_graph.node_weights_mut());

    if tick == max {
        return (base_score, vec![])
    }


    let mut max_score = 0;
    let mut best_path = Vec::new();
    let this_valve = valve_graph.node_weight_mut(current_location).unwrap();
    if !this_valve.is_on && this_valve.flow_rate != 0 {
        this_valve.is_on = true;
        (max_score, best_path) = branch(valve_graph, tick + 1, current_location, max, distances);
        best_path.push((current_location, tick));
        // Reset state
        let this_valve = valve_graph.node_weight_mut(current_location).unwrap();
        this_valve.is_on = false;
    }
    else {
        let nodes = (0..valve_graph.node_count() as u32)
            .map(|n| NodeIndex::from(n))
            .filter(|n| {
                let v = valve_graph.node_weight(*n).unwrap();
                !v.is_on &&
                v.flow_rate != 0 &&
                tick + distances[&(current_location, *n)] as u8 <= max &&
                *n != current_location
            })
            .map(|n| (n, distances[&(current_location, n)] as u8))
            .collect::<HashMap<_, _>>();

        for key in nodes.keys() {

            let (mut score, path) = branch(valve_graph, tick + nodes[key], *key, max, distances);
            score += base_score * (nodes[key] - 1) as u32;
            if score > max_score {
                max_score = score;
                best_path = path;
            }
        }
    }
    if max_score == 0 {
        // Nothing more to do, wait it out
        max_score = base_score * (max - tick) as u32;
    }

    let total = base_score + max_score;
    (total, best_path)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Movement {
    pub current_loc: NodeIndex,
    pub dest: Option<NodeIndex>,
    pub time_left: i32,
}

const P1_TIME_MAX: u8 = 30;
const ELY_TIME_MAX: u8 = 26;
/* 
fn branch_with_elephant_alone(valve_graph: &mut Graph<Valve, (), Directed>, tick: u8, me: Movement, mut ely: Movement) 
    -> (u32, VecDeque<((Movement, Movement), u8)>) 
{
    assert!(tick < ELY_TIME_MAX);
    //return branch_with_elephant(valve_graph, tick + 1, me, ely);

    let path_info = ((me, ely), tick);
    let base_score = 0;//score_valves_on_tick(valve_graph.node_weights_mut());

    if ely.dest.is_some() {
        if ely.time_left == 0 {
            ely.current_loc = ely.dest.unwrap();
            ely.dest = None;
        } else {
            ely.time_left -= 1;
            //let ely_dest = ely.dest.unwrap();
            let (score, mut path) = branch_with_elephant(valve_graph, tick + 1, me, ely);
            path.push_front(path_info);
            return (score + base_score, path)
        }
    }


    //let me_loc = me.current_loc;
    let ely_loc = ely.current_loc;

    let mut max_score = 0;
    let mut best_path = VecDeque::new();
    let this_valve = valve_graph.node_weight_mut(ely.current_loc).unwrap();
    if !this_valve.is_on && this_valve.flow_rate != 0 {
        this_valve.is_on = true;  
        (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely);
        // Reset state
        let this_valve = valve_graph.node_weight_mut(ely_loc).unwrap();
        this_valve.is_on = false;
    }

    else {
        //let this_valve = valve_graph.node_weight(me.loc).unwrap();
        let distances_to_valves = dijkstra(&*valve_graph, ely.current_loc, 
            None, |_| 1);
        let mut move_scores = HashMap::new();
        for (key, dis) in distances_to_valves.iter() {
            if *key == ely.current_loc {
                continue;
            } 
            if me.dest.is_some() && *key == me.dest.unwrap() {
                continue;
            }
            //else if *key == me.current_loc {
            //    continue;
            //}
            let valve = valve_graph.node_weight(*key).unwrap();
            if !valve.is_on && valve.flow_rate != 0 && tick + dis < (ELY_TIME_MAX - 1)  {
                move_scores.insert(*key, dis);
            }
        }
        if move_scores.keys().is_empty() { 
            println!("Keys are empty!");
            println!("Tick: {}", tick);
            println!("{:?}", Dot::with_config(&*valve_graph, &[]));
        }
        for key in move_scores.keys() {
            let new_ely = Movement {
                current_loc: ely.current_loc,
                dest: Some(*key),
                time_left: *move_scores[key] as i32 - 1,
            };

            let (score, path) = branch_with_elephant(valve_graph, tick + 1, 
                me.clone(), new_ely.clone());
            if score > max_score {
                max_score = score;
                best_path = path;
                //decision = format!("Ely Moving on new path: {:?} with distance left {}", new_ely.dest.unwrap(), new_ely.time_left);
            }
        }
    }
    
    //if max_score == 0 {
    //    // Nothing more to do, wait it out
    //    //decision = String::from("Ely Nothing to do, waiting");
    //    (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely);
    //}

    //me_decision.push_str(&decision);
    best_path.push_front(path_info);
    //let total = base_score + max_score;
    (max_score, best_path)
}

fn branch_with_elephant(valve_graph: &mut Graph<Valve, (), Directed>, tick: u8, mut me: Movement, ely: Movement)
-> (u32, VecDeque<((Movement, Movement), u8)>) {
    
    let path_info = ((me, ely), tick);

    if tick > ELY_TIME_MAX {
        return (0, VecDeque::from([path_info]));
    }

    let me_loc = me.current_loc;
    

    let base_score = score_valves_on_tick(valve_graph.node_weights_mut());

    if tick == ELY_TIME_MAX {
        return (base_score, VecDeque::from([path_info]))
    }

    // If we're still traveling, the ely can move more or less independently
    if me.dest.is_some() {
        if me.time_left == 0 {
            me.current_loc = me.dest.unwrap();
            me.dest = None;
        } else {
            me.time_left -= 1;
            //let me_dest = me.dest.unwrap();
            let (score, path) = branch_with_elephant_alone(valve_graph, tick, me, ely, 
            );
            return (score + base_score, path)
        }
    }
    let path_info = ((me, ely), tick);


    let mut best_path = VecDeque::new();
    //best_path.push_front(((me_loc, ely_loc), tick));

    // Same idea above goes for if we've reached our destination
    let mut max_score = 0;
    
    //let mut decision = String::from("");
    let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
    if !this_valve.is_on && this_valve.flow_rate != 0 {
        this_valve.is_on = true;  
        //let decision = String::from("me opening valve ");     
        (max_score, best_path) = branch_with_elephant_alone(valve_graph, tick, me, ely);
        
        // Reset state
        let this_valve = valve_graph.node_weight_mut(me_loc).unwrap();
        this_valve.is_on = false;
    }

    else {
        //let this_valve = valve_graph.node_weight(me.loc).unwrap();
        let distances_to_valves = dijkstra(&*valve_graph, me.current_loc, 
            None, |_| 1);
        let mut move_scores = HashMap::new();
        for (key, dis) in distances_to_valves.iter() {
            if *key == me.current_loc {
                continue;
            } 
            if ely.dest.is_some() && *key == ely.dest.unwrap() {
                continue;
            }
            else if *key == ely.current_loc {
                continue;
            }
            let valve = valve_graph.node_weight(*key).unwrap();
            if !valve.is_on && valve.flow_rate != 0 && tick + dis < (ELY_TIME_MAX - 1)   {
                move_scores.insert(*key, dis);
            }
        }
        
        let ely_valve = valve_graph.node_weight_mut(ely.current_loc).unwrap();
        if (ely.dest.is_some() && ely.time_left > 0) || (!ely_valve.is_on && ely_valve.flow_rate != 0) {
            for key in move_scores.keys() {

                let new_me = Movement {
                    current_loc: me.current_loc,
                    dest: Some(*key),
                    time_left: *move_scores[key] as i32 - 1,
                };
    
                //let decision = format!("Moving on new path: {:?} with distance left {}", new_me.dest.unwrap(), new_me.time_left);            
                let (score, path) = branch_with_elephant_alone(valve_graph, tick, 
                    new_me.clone(), ely.clone());
                if score > max_score {
                    max_score = score;
                    best_path = path;
                }
            }
        } 
        else {
            let combos = move_scores.keys().combinations_with_replacement(2);
            let combos = combos.filter(|key_list| *key_list[0] != *key_list[1]);
            let combos = combos.map(|mut key_list| {
                if key_list[0] < key_list[1] {
                    key_list.swap(0, 1);
                }
                key_list
            });
            let combos = combos.sorted_by(|list1, list2| {
                let first_cmp = (*list1[0]).cmp(list2[0]);
                if first_cmp == Ordering::Equal {
                    (*list1[1]).cmp(list2[1])
                } else { first_cmp }
            });

            // Find sim dests for both
            let mut key_num = 1;
            let num_pairs = combos.clone().count();
            for key_list in combos {
                if tick == 1 {
                    println!("Key {} out of {}", key_num, num_pairs)
                }

                let me_key = key_list[0];
                let ely_key = key_list[1];

                let new_me = Movement {
                    current_loc: me.current_loc,
                    dest: Some(*me_key),
                    time_left: *move_scores[me_key] as i32 - 1,
                };

                let new_ely = Movement {
                    current_loc: me.current_loc,
                    dest: Some(*ely_key),
                    time_left: *move_scores[ely_key] as i32 - 1,
                };
    
                let (score, mut path) = branch_with_elephant(valve_graph, tick + 1, 
                    new_me.clone(), new_ely.clone());
                path.push_front(path_info);

                if tick == 1 {
                    println!("nodes: {:?}", key_list);
                    println!("path:");
                    for path_node in &path {
                        println!("{:?}", path_node);
                    }
                    println!("Score: {}", score);
                }

                if score > max_score {
                    max_score = score;
                    best_path = path;
                    //decision = format!("Moving on new path: {:?} with distance left {}\nEly moving on new path {:?} with distance left {}", 
                    //new_me.dest.unwrap(), new_me.time_left, new_ely.dest.unwrap(), new_ely.time_left);
                }
                key_num += 1;
            }
        }
    }
    
    if max_score == 0 {
        // Nothing more to do, wait it out
        //decision = String::from("Me Nothing to do, waiting");
        //(max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely);
        let mut info = path_info;
        for tick in tick..(ELY_TIME_MAX + 1) {
            info.1 = tick;
            max_score += base_score;
            best_path.push_back(info);
        }
    }

    //if decision.as_str() != "" {
    //    best_path.push_front(((me_loc, ely_loc), tick));
    //}
    let total = base_score + max_score;
    (total, best_path)
}
*/

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Decision {
    MoveAlongPath(Option<NodeIndex>),
    TurnOnValve(Option<NodeIndex>),
    PickNewDest(Option<NodeIndex>),
    NothingToDo,
}

fn make_decision(valve_graph: &mut Graph<Valve, (), Directed>, tick: u8, subject: &mut Movement) -> Decision {
    // Are we still moving?
    if subject.dest.is_some() {
        if subject.time_left <= 0 {
            subject.current_loc = subject.dest.unwrap();
            subject.dest = None;
        } else {
            return Decision::MoveAlongPath(subject.dest);
        }
    }
    // Are we going to turn on a valve?
    let this_valve = valve_graph.node_weight(subject.current_loc).unwrap();
    if !this_valve.is_on && this_valve.flow_rate != 0 {
        Decision::TurnOnValve(Some(subject.current_loc))

    } else { 
        // Can we reach any actual valves?
        //for idx in 0..(valve_graph.node_count() as u32) {
        //    let node = NodeIndex::from(idx);
        //    let valve = valve_graph.node_weight(node).unwrap();
        //    if !valve.is_on &&
        //    valve.flow_rate != 0 &&
        //    distances[&(subject.current_loc, node)] as u8 <= tick + ELY_TIME_MAX {
        //        return Decision::PickNewDest(None)
        //    }
        //}
        //Decision::NothingToDo
        Decision::PickNewDest(None)
    }

}

// REMEMBER THIS FUNCTION HAS BEEN TURNED OFF   
fn is_better(p1: (i32, i32), p2: (i32, i32)) -> bool {
    //return false;

    if p1.0 < p2.0 {
        p1.1 <= p2.1
    }
    else if p1.1 < p2.1 {
        p1.0 <= p2.0
    }
    else { p1.0 == p2.0 && p1.1 == p2.1 }
}

static HMAP_NO_INSERT_TICK: u8 = 0;
static INITIAL_HMAP_CAPACITY: usize = 0;

fn branch_with_elephant<'a>(valve_graph: &mut Graph<Valve<'a>, (), Directed>, tick: u8, mut me: Movement, mut ely: Movement,
states: &mut HashMap<(ValveState, NodeIndex, NodeIndex, u8), (u32, Vec<(u8, Decision, Decision)>)>, 
distances: &HashMap<(NodeIndex, NodeIndex), i32>) -> (u32, Vec<(u8, Decision, Decision)>) 
{

    let mut ely_decision = make_decision(valve_graph, tick, &mut ely);
    let mut my_decision = make_decision(valve_graph, tick, &mut me);
    
    //println!("me: {}, {:?}", me.time_left, me.dest);
    //println!("ely: {}, {:?}", ely.time_left, ely.dest);
    

    if tick > ELY_TIME_MAX {
        return (0, Vec::new());
    }
    

    let base_score = score_valves_on_tick(valve_graph.node_weights_mut());

    if tick == ELY_TIME_MAX {        
        return (base_score, Vec::new())
    }

    let (max_score, mut best_path) = match ely_decision {
        Decision::MoveAlongPath(_) => {
            // Move ely along path
            // We don't need to look at dest because make_decision() already does that for us
            ely.time_left -= 1;

            // Make my decision
            // If we're still traveling, the ely can move more or less independently
            if me.dest.is_some() {
                me.time_left -= 1;
                let (score, mut path) = branch_with_elephant(valve_graph, tick + 1, me, ely, 
                states, distances);
                path.push((tick, my_decision, ely_decision));
                return (score + base_score, path)
            }       
            // Same idea above goes for if we've reached our destination
            let mut max_score = 0;
            let mut best_path = Vec::new();
            
            //let mut decision = String::from("");
            let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
            if !this_valve.is_on && this_valve.flow_rate != 0 {
                this_valve.is_on = true;  
                (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely, states, distances);
                
                // Reset state
                let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
                this_valve.is_on = false;
            }
        
            else {
                let nodes = (0..valve_graph.node_count() as u32)
                    .map(|n| NodeIndex::from(n))
                    .filter(|n| {
                        let v = valve_graph.node_weight(*n).unwrap();
                        !v.is_on &&
                        v.flow_rate != 0 &&
                        tick + distances[&(me.current_loc, *n)] as u8 <= ELY_TIME_MAX &&
                        *n != me.current_loc &&
                        *n != ely.dest.unwrap()
                    })
                    .map(|n| (n, distances[&(me.current_loc, n)] as u8))
                    .collect::<HashMap<_, _>>();
    
                for key in nodes.keys() {
    
                    let new_me = Movement {
                        current_loc: me.current_loc,
                        dest: Some(*key),
                        time_left: nodes[key] as i32 - 1,
                    };
        
                    //let decision = format!("Moving on new path: {:?} with distance left {}", new_me.dest.unwrap(), new_me.time_left);            
                    let (score, path) = branch_with_elephant(valve_graph, tick + 1, 
                        new_me, ely, states, distances);
                    if score > max_score {
                        max_score = score;
                        best_path = path;
                        my_decision = Decision::PickNewDest(Some(*key));
                    }
                }
            }
            
            if max_score == 0 {
                (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely, states, distances);
            }
            (max_score, best_path)
        },
        Decision::TurnOnValve(_) => {
            // Make ely turn on valve
            let ely_valve = valve_graph.node_weight_mut(ely.current_loc).unwrap();
            ely_valve.is_on = true;                  
            
            // Sim my decision
            if me.dest.is_some() {
                me.time_left -= 1;
                //let me_dest = me.dest.unwrap();
                let (score, mut path) = branch_with_elephant(valve_graph, tick + 1, me, ely, 
                states, distances);
                path.push((tick, my_decision, ely_decision));
                // Make sure to reset ely valve
                let ely_valve = valve_graph.node_weight_mut(ely.current_loc).unwrap();
                ely_valve.is_on = false;
                return (score + base_score, path)
            } 
        
            // Same idea above goes for if we've reached our destination
            let mut max_score = 0;
            let mut best_path = Vec::new();

            //let mut decision = String::from("");
            let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
            if !this_valve.is_on && this_valve.flow_rate != 0 {
                this_valve.is_on = true;  
                //let decision = String::from("me opening valve ");     
                (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely, states, distances);
                
                // Reset state
                let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
                this_valve.is_on = false;
            }
        
            else {
                let nodes = (0..valve_graph.node_count() as u32)
                    .map(|n| NodeIndex::from(n))
                    .filter(|n| {
                        let v = valve_graph.node_weight(*n).unwrap();
                        !v.is_on &&
                        v.flow_rate != 0 &&
                        tick + distances[&(me.current_loc, *n)] as u8 <= ELY_TIME_MAX &&
                        *n != me.current_loc
                    })
                    .map(|n| (n, distances[&(me.current_loc, n)] as u8))
                    .collect::<HashMap<_, _>>();

                for key in nodes.keys() {
    
                    let new_me = Movement {
                        current_loc: me.current_loc,
                        dest: Some(*key),
                        time_left: nodes[key] as i32 - 1,
                    };
        
                    //let decision = format!("Moving on new path: {:?} with distance left {}", new_me.dest.unwrap(), new_me.time_left);            
                    let (score, path) = branch_with_elephant(valve_graph, tick + 1, 
                        new_me, ely, states, distances);
                    if score > max_score {
                        max_score = score;
                        best_path = path;
                        my_decision = Decision::PickNewDest(Some(*key));
                    }
                }
                
            }
            
            if max_score == 0 {
                (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely, states, distances);
            }
            // Make sure to reset ely valve
            let ely_valve = valve_graph.node_weight_mut(ely.current_loc).unwrap();
            ely_valve.is_on = false;
            
            (max_score, best_path)
        },
        Decision::PickNewDest(_) => {
            let mut max_score = 0;
            let mut best_path = Vec::new();
            if my_decision == Decision::PickNewDest(None) {
                // FIXME: HASHMAP CAUSES US TO GET THE WRONG ANSWER
                
                let valve_state = ValveState::new(valve_graph);
                let normal_key = (valve_state, me.current_loc, ely.current_loc, tick);
                let reversed_key = (valve_state, ely.current_loc, me.current_loc, tick);
                if states.contains_key(&normal_key) {
                    //println!("Hit cache at depth {}", tick);
                    (max_score, best_path) = states[&normal_key].clone();
                }                
                else if states.contains_key(&reversed_key) {
                    (max_score, best_path) = states[&reversed_key].clone();
                }
                else {
                    let nodes = (0..valve_graph.node_count() as u32)
                    .map(|n| NodeIndex::from(n))
                    .filter(|n| {
                        let v = valve_graph.node_weight(*n).unwrap();
                        !v.is_on &&
                        v.flow_rate != 0 &&
                        true
                        //*n != me.current_loc &&
                        //*n != ely.current_loc
                    })
                    .collect_vec();

                    if tick == 16 {
                        //println!("{:?}", Dot::with_config(&*valve_graph, &[]));
                    }

                    let combos = nodes.iter().combinations_with_replacement(2);
                    let combos = combos.filter(|key_list| *key_list[0] != *key_list[1]);
                    let mut key_num = 1;
                    let num_pairs = combos.clone().count();
                    let combos = combos.map(|key_list| {
                        let me_dis_key_1 = distances[&(me.current_loc, *key_list[0])];
                        let me_dis_key_2 = distances[&(me.current_loc, *key_list[1])];
                        let ely_dis_key_1 = distances[&(ely.current_loc, *key_list[0])];
                        let ely_dis_key_2 = distances[&(ely.current_loc, *key_list[1])];
                        (key_list, (me_dis_key_1, ely_dis_key_2), (ely_dis_key_1, me_dis_key_2))
                    });
                    // sort buy "best", maybe that helps cache
                    let combos = combos
                    .sorted_by(|(k1, _, _), (k2, _, _)| 
                    {
                        let v1 = valve_graph.node_weight(*k2[0]).unwrap();
                        let v2 = valve_graph.node_weight(*k2[1]).unwrap();
                        let v3 = valve_graph.node_weight(*k1[0]).unwrap();
                        let v4 = valve_graph.node_weight(*k1[1]).unwrap();
                        
                        //(v3.flow_rate + v4.flow_rate).cmp(&(v1.flow_rate + v2.flow_rate))
                        (v1.flow_rate + v2.flow_rate).cmp(&(v3.flow_rate + v4.flow_rate))
                    });
                    for (key_list, p1, p2) in combos {
                        // Try to pick best key pair
                        let me_dis_key_1 = p1.0;
                        let ely_dis_key_2 = p1.1;
                        let ely_dis_key_1 = p2.0;
                        let me_dis_key_2 = p2.1;
                        //let p1 = (me_dis_key_1, ely_dis_key_2);
                        //let p2 = (ely_dis_key_1, me_dis_key_2);
                        let (score, path, me_key, ely_key) = 
                        if is_better(p1, p2) {
                            if tick + p1.0 as u8 >= ELY_TIME_MAX && tick + p1.1 as u8 >= ELY_TIME_MAX { continue; }

                            let me_key = key_list[0];
                            let ely_key = key_list[1];

                            let new_me = Movement {
                                current_loc: me.current_loc,
                                dest: Some(*me_key),
                                time_left: me_dis_key_1 - 1,
                            };

                            let new_ely = Movement {
                                current_loc: ely.current_loc,
                                dest: Some(*ely_key),
                                time_left: ely_dis_key_2 - 1,
                            };
                            let (score, path) = branch_with_elephant(valve_graph, tick + 1, 
                                new_me.clone(), new_ely.clone(), states, distances);
                            //println!("Distances: {}, {}, {}, {}", me_dis_key_1.0, ely_dis_key_2.0, ely_dis_key_1.0, me_dis_key_2.0);
                            //
                            //println!("Chose me to key 1 and ely to key 2");
                            
                            (score, path, me_key, ely_key)
                        }
                        else if is_better(p2, p1) {
                            if tick + p2.0 as u8 >= ELY_TIME_MAX && tick + p2.1 as u8 >= ELY_TIME_MAX { continue; }

                            let me_key = key_list[1];
                            let ely_key = key_list[0];

                            let new_me = Movement {
                                current_loc: me.current_loc,
                                dest: Some(*me_key),
                                time_left: me_dis_key_2 - 1,
                            };

                            let new_ely = Movement {
                                current_loc: ely.current_loc,
                                dest: Some(*ely_key),
                                time_left: ely_dis_key_1 - 1,
                            };
                            let (score, path) = branch_with_elephant(valve_graph, tick + 1, 
                                new_me.clone(), new_ely.clone(), states, distances);
                            //println!("Distances: {}, {}, {}, {}", me_dis_key_1.0, ely_dis_key_2.0, ely_dis_key_1.0, me_dis_key_2.0);
                            //println!("Chose ely to key 1 and me to key 2");

                            (score, path, me_key, ely_key)
                        }
                        else {
                            // We have to try both options
                            // FIXME REMEMEBER OR INSTEAD OF AND
                            let me_key_1 = key_list[0];
                            let ely_key_2 = key_list[1];

                            let new_me_1 = Movement {
                                current_loc: me.current_loc,
                                dest: Some(*me_key_1),
                                time_left: me_dis_key_1 - 1,
                            };

                            let new_ely_1 = Movement {
                                current_loc: ely.current_loc,
                                dest: Some(*ely_key_2),
                                time_left: ely_dis_key_2 - 1,
                            };
                            let (score1, path1) = 
                            if ((tick + p1.0 as u8) < ELY_TIME_MAX || (tick + p1.1 as u8) < ELY_TIME_MAX) {
                                branch_with_elephant(valve_graph, tick + 1, 
                                    new_me_1.clone(), new_ely_1.clone(), states, distances)
                            }
                            else {
                                (0, Vec::new())
                            };
                        
                            let me_key_2 = key_list[1];
                            let ely_key_1 = key_list[0];

                            let new_me_2 = Movement {
                                current_loc: me.current_loc,
                                dest: Some(*me_key_2),
                                time_left: me_dis_key_2 - 1,
                            };

                            let new_ely_2 = Movement {
                                current_loc: ely.current_loc,
                                dest: Some(*ely_key_1),
                                time_left: ely_dis_key_1 - 1,
                            };
                            let (score2, path2) = 
                            if ((tick + p2.0 as u8) < ELY_TIME_MAX || (tick + p2.1 as u8) < ELY_TIME_MAX) {
                                branch_with_elephant(valve_graph, tick + 1, 
                                    new_me_2.clone(), new_ely_2.clone(), states, distances)
                            }
                            else {
                                (0, Vec::new())
                            };

                            //println!("Distances: {}, {}, {}, {}", me_dis_key_1.0, ely_dis_key_2.0, ely_dis_key_1.0, me_dis_key_2.0);
                            //println!("Simmed both options");
                            if score1 > score2 {
                                //rintln!("Chose me to key 1 and ely to key 2");
                                (score1, path1, me_key_1, ely_key_2)
                            }
                            else if score2 != 0 {
                                //println!("Chose ely to key 1 and me to key 2");
                                (score2, path2, me_key_2, ely_key_1)
                            }
                            else {
                                continue;
                            }
                        };

                        // TODO REMOVE CLONE ONCE DONE DEBUGGING
                        if score > max_score {
                            max_score = score;
                            best_path = path.clone();
                            my_decision = Decision::PickNewDest(Some(*me_key));
                            ely_decision = Decision::PickNewDest(Some(*ely_key));
                        }

                        if tick == 1 {
                        //(tick == 23 && valve_graph.node_weight(NodeIndex::from(4)).unwrap().is_on
                        //&& me.current_loc == NodeIndex::from(4)) {
                            println!("Key {} out of {}, max {}", key_num, num_pairs, max_score);
                            //println!("My node: {}", valve_graph.node_weight(*me_key).unwrap().name);
                            //println!("Ely node: {}", valve_graph.node_weight(*ely_key).unwrap().name);
                            //println!("Score: {}", score);
                            //println!("Path:");
                            //for (tick, me, ely) in path.iter().rev() {
                            //    println!("{}: {:?}, {:?}", tick, me, ely);
                            //}
                            //println!("---------------------");
                        }

                        
                        key_num += 1;
                    }         
                
                    if max_score == 0 {
                        (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely, states, distances);
                    }

                    // WHY DOES THIS WORK?
                    if tick < HMAP_NO_INSERT_TICK {
                        states.insert(normal_key, (max_score, best_path.clone()));
                        states.insert(reversed_key, (max_score, best_path.clone()));
                    }
                }
            }
            else {
                // We do other stuff
                // Again, no need to check if we've reached our destination
                let me_has_dest = me.dest.is_some();
                if me.dest.is_some() {
                    me.time_left -= 1;
                } 
                else {
                    // We're turning on our valve
                    let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
                    this_valve.is_on = true;  
                }
                
                // Ely picks dest
                let nodes = (0..valve_graph.node_count() as u32)
                    .map(|n| NodeIndex::from(n))
                    .filter(|n| {
                        let v = valve_graph.node_weight(*n).unwrap();
                        !v.is_on &&
                        v.flow_rate != 0 &&
                        tick + distances[&(ely.current_loc, *n)] as u8 <= ELY_TIME_MAX &&
                        *n != ely.current_loc &&
                        if me_has_dest { *n != me.dest.unwrap() } else {true}
                    })
                    .map(|n| (n, distances[&(ely.current_loc, n)] as u8))
                    .collect::<HashMap<_, _>>();
                
                if tick == 20 {
                    //println!("{:?}", Dot::with_config(&*valve_graph, &[]));
                }

                for key in nodes.keys() {
                    let new_ely = Movement {
                        current_loc: ely.current_loc,
                        dest: Some(*key),
                        time_left: nodes[key] as i32 - 1,
                    };
        
                    //let decision = format!("Moving on new path: {:?} with distance left {}", new_me.dest.unwrap(), new_me.time_left);            
                    let (score, path) = branch_with_elephant(valve_graph, tick + 1, 
                        me, new_ely, states, distances);
                    // TODO REMOVE CLONE
                    if score > max_score {
                        max_score = score;
                        best_path = path.clone();
                        ely_decision = Decision::PickNewDest(Some(*key));
                    }
                    if tick == 20 {
                        //println!("Ely node: {}, max score {}", valve_graph.node_weight(*key).unwrap().name, max_score);
                        //println!("Score: {}", score);
                        //println!("Path:");
                        //for (tick, me, ely) in path.iter().rev() {
                        //    println!("{}: {:?}, {:?}", tick, me, ely);
                        //}
                        //println!("---------------------");
                    }
                }

                if max_score == 0 {
                    (max_score, best_path) = branch_with_elephant(valve_graph, tick + 1, me, ely, states, distances);
                }
                // Reset state
                if me.dest.is_none() {
                    let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
                    this_valve.is_on = false;
                }
            }

            (max_score, best_path)
        },
        _ => unreachable!(),
    };

    if max_score == 0 {
        //best_path.pop();
        //best_path.push((tick, Decision::NothingToDo, Decision::NothingToDo));
        //// Account for the remaining rounds
        //max_score = (ELY_TIME_MAX - tick) as u32 * base_score;
        println!("{}", tick);
        println!("Me: {:?}, decision: {:?}", me, my_decision);
        println!("Ely: {:?}, decision: {:?}", ely, ely_decision);
        println!("Best path: {:?}", best_path);
        println!("{:?}", Dot::with_config(&*valve_graph, &[]));
        panic!();
    }
    
    best_path.push((tick, my_decision, ely_decision));

    (base_score + max_score, best_path)
}

/* 
fn sim_binomial<'a>(valve_graph: &mut Graph<Valve<'a>, (), Directed>, start: NodeIndex, nodes: 
Vec<NodeIndex>, distances: &HashMap<(NodeIndex, NodeIndex), i32>) -> u32 {
    let num_intersting_nodes = valve_graph.node_weights()
        .filter(|v| v.flow_rate != 0)
        .count();
    let num_hum = num_intersting_nodes.div_ceil(2);
    let num_ely = num_intersting_nodes - num_hum;
    let hum_options_dup = nodes.iter()
        .filter(|n| valve_graph.node_weight(**n).unwrap().flow_rate != 0)
        // This could be made better because we need unordered and this does ordered
        .combinations_with_replacement(num_hum) 
        .map(|v| v.into_iter().collect::<HashSet<_>>())
        .collect_vec();
    let mut hum_options = Vec::new();
    for map in hum_options_dup {
        if !hum_options.contains(&map) {
            hum_options.push(map);
        }
    }
    //let mut max = 0;
    let mut count = 0;
    println!("Generated options");
    hum_options.iter().map(|option| {
        println!("{} out of {}", count, hum_options.len());
        let mut hum_graph = valve_graph.clone();
        for node in &nodes {
            if !option.contains(&node) {
                hum_graph.node_weight_mut(*node).unwrap().flow_rate = 0;
            }
        }
        let mut ely_graph = valve_graph.clone();
        for node in &nodes {
            if option.contains(&node) {
                hum_graph.node_weight_mut(*node).unwrap().flow_rate = 0;
            }
        }
        let score = branch(&mut hum_graph, 1, start, ELY_TIME_MAX, distances).0
        + branch(&mut ely_graph, 1, start, ELY_TIME_MAX, distances).0;
        count += 1;
        score
    }).max().unwrap()

    /* 
    for option in &hum_options {
        println!("{} out of {}", count, hum_options.len());
        let mut hum_graph = valve_graph.clone();
        for node in &nodes {
            if !option.contains(&node) {
                hum_graph.node_weight_mut(*node).unwrap().flow_rate = 0;
            }
        }
        let mut ely_graph = valve_graph.clone();
        for node in &nodes {
            if option.contains(&node) {
                hum_graph.node_weight_mut(*node).unwrap().flow_rate = 0;
            }
        }
        let score = branch(&mut hum_graph, 1, start, ELY_TIME_MAX).0
        + branch(&mut ely_graph, 1, start, ELY_TIME_MAX).0;
        if score > max {
            max = score;
        }
        count += 1;
    }
    max
    */
}
*/

fn follow_path(mut valve_graph: Graph<Valve, (), Directed>, path: &Vec<(NodeIndex, u8)>) {
    let mut it = path.iter().rev().peekable();
    let mut relieved = 0;
    for tick in 1..31 {
        let thing = score_valves_on_tick(valve_graph.node_weights_mut());
        relieved += thing;
        //println!("tick {tick} actual score is {relieved}");
        println!("Relieved {} pressure", thing);

        if it.peek().is_some() && it.peek().unwrap().1 == tick {
            let node = it.peek().unwrap().0;
            valve_graph.node_weight_mut(node).unwrap().is_on = true;
            it.next();
        }
    }
}

fn score_route(route: &Vec<(u8, Decision, Decision)>, mut valve_graph: Graph<Valve, (), Directed>,
mut me: Movement, mut ely: Movement) {
    println!("Scoring route");
    let mut score = 0;
    let mut stopped = false;
    for tick in 1..=ELY_TIME_MAX {
        let tick_score = score_valves_on_tick(valve_graph.node_weights_mut());
        println!("{}: {}", tick, tick_score);
        score += tick_score;

        if route.len() >= tick.into() && !stopped {
            let (other_tick, me_decision, ely_decision) = route[route.len() - tick as usize];
            if me_decision == Decision::NothingToDo {
                assert_eq!(ely_decision, Decision::NothingToDo);
                stopped = true;
                continue;
            }
            assert_eq!(tick, other_tick);
            //if me.dest.is_some() && me.time_left <= 0 {
            //    me.current_loc = me.dest.unwrap();
            //    me.dest = None;
            //}
            //if ely.dest.is_some() && ely.time_left <= 0 {
            //    ely.current_loc = ely.dest.unwrap();
            //    ely.dest = None;
            //}
            match me_decision {
                Decision::MoveAlongPath(_) => (), // Should be able to ignore this
                Decision::PickNewDest(dest) => me.dest = dest,
                Decision::TurnOnValve(_) => {
                    me.current_loc = me.dest.unwrap();
                    //me.dest = None;
                    let this_valve = valve_graph.node_weight_mut(me.current_loc).unwrap();
                    this_valve.is_on = true;
                },
                _ => unreachable!(),
            }

            match ely_decision {
                Decision::MoveAlongPath(_) => (), // Should be able to ignore this
                Decision::PickNewDest(dest) => ely.dest = dest,
                Decision::TurnOnValve(_) => {
                    ely.current_loc = ely.dest.unwrap();
                    //ely.dest = None;
                    let this_valve = valve_graph.node_weight_mut(ely.current_loc).unwrap();
                    this_valve.is_on = true;
                },
                _ => unreachable!(),
            }
        }
    }
    println!("Final score: {}", score)
}

pub fn part1(input: &str) {
    let mut valve_graph: Graph<Valve, (), Directed> = Graph::new();
    let mut nodes = Vec::new();

    for line in input.split("\n") {
        let valve = make_valve(line);
        let n = valve_graph.add_node(valve);
        nodes.push(n);
    }
    // Make connections
    for node in &nodes {
        for other_node in &nodes {
            if node == other_node { continue; }
            let valve = valve_graph.node_weight(*node).unwrap();
            let other_valve = valve_graph.node_weight(*other_node).unwrap();
            if valve.connections.contains(&other_valve.name) {
                valve_graph.extend_with_edges(&[(*node, *other_node)]);
            }
        }
    }
    for valve in valve_graph.node_weights_mut() {
        valve.connections.clear();
    }
    let distances = floyd_warshall(&valve_graph, |_| 1).unwrap();

    //println!("{:?}", Dot::with_config(&valve_graph, &[]));
    let right_node = nodes.iter().find(|v| {
        let valve = valve_graph.node_weight(**v).unwrap();
        valve.name == "AA"
    }).unwrap();

    let (max, path) = branch(&mut valve_graph, 1, *right_node, P1_TIME_MAX, &distances);
    println!("{}", max);
    
    //follow_path(valve_graph.clone(), &path);
}

fn test_pt2(valve_graph: &mut Graph<Valve, (), Directed>, 
states: &mut HashMap<(ValveState, NodeIndex, NodeIndex, u8), (u32, Vec<(u8, Decision, Decision)>)>,
distances: &HashMap<(NodeIndex, NodeIndex), i32>) 
{
    // TICK AT 11
    // HUMAN HAS JUST REACHED IA
    // ELY HAS JUST REACHED LA
    let turn_on = ["AA", "FA", "GA", "HA"];
    for valve in valve_graph.node_weights_mut() {
        if turn_on.contains(&valve.name) {
            valve.is_on = true;
        }
    }
    let mut right_me = None;
    for node in 0..(valve_graph.node_count() as u32) {
        if valve_graph.node_weight(NodeIndex::from(node)).unwrap().name == "IA" {
            right_me = Some(node);
            break;
        }
    }
    let right_me = right_me.unwrap();
    let me = Movement { 
        current_loc: NodeIndex::from(right_me),
        dest: None,
        time_left: 0,
    };
    let mut right_ely = None;
    for node in 0..(valve_graph.node_count() as u32) {
        if valve_graph.node_weight(NodeIndex::from(node)).unwrap().name == "LA" {
            right_ely = Some(node);
            break;
        }
    }
    let right_ely = right_ely.unwrap();
    let ely = Movement { 
        current_loc: NodeIndex::from(right_ely),
        dest: None,
        time_left: 0,
    };
    let (_, path) = branch_with_elephant(valve_graph, 11, me, ely, states, distances);
    for (tick, me, ely) in path.iter().rev() {
        println!("{}: {:?}, {:?}", tick, me, ely);
    }
    //score_route(&path, valve_graph.clone(), me, ely);

}

pub fn part2(input: &str) {
    let mut valve_graph: Graph<Valve, (), Directed> = Graph::new();
    let mut states = HashMap::with_capacity(INITIAL_HMAP_CAPACITY);
    let mut nodes = Vec::new();

    for line in input.split("\n") {
        let valve = make_valve(line);
        let n = valve_graph.add_node(valve);
        nodes.push(n);
    }
    // Make connections
    for node in &nodes {
        for other_node in &nodes {
            if node == other_node { continue; }
            let valve = valve_graph.node_weight(*node).unwrap();
            let other_valve = valve_graph.node_weight(*other_node).unwrap();
            if valve.connections.contains(&other_valve.name) {
                valve_graph.extend_with_edges(&[(*node, *other_node)]);
            }
        }
    }
    for valve in valve_graph.node_weights_mut() {
        valve.connections.clear();
    }

    let distances = floyd_warshall(&valve_graph, |_| 1).unwrap();

    //println!("{:?}", Dot::with_config(&valve_graph, &[]));
    let right_node = nodes.iter().find(|v| {
        let valve = valve_graph.node_weight(**v).unwrap();
        valve.name == "AA"
    }).unwrap();

    let me = Movement { 
        current_loc: *right_node,
        dest: None,
        time_left: 0,
    };

    let ely = me.clone();
    //println!("{:?}", nodes);
    //println!("{:?}", valve_graph.node_weights().map(|v| v.name).collect_vec());
    //
    //test_pt2(&mut valve_graph, &mut states, &distances);
    let (max, path) = branch_with_elephant(&mut valve_graph, 1, me, ely, &mut states, &distances);
    //let max = sim_binomial(&mut valve_graph, *right_node, nodes, &distances);
    println!("{}", max);
    //for (tick, me, ely) in path.iter().rev() {
    //    println!("{}: {:?}, {:?}", tick, me, ely);
    //}
    //score_route(&path, valve_graph.clone(), me, ely);
}

pub fn day16(input: &str) {
    //part1(input);
    part2(input);
}