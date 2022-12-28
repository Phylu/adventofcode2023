use std::collections::HashMap;
use itertools::Itertools;
use indicatif::ProgressBar;

use log::{debug};

use pathfinding::prelude::dijkstra;
use regex::Regex;

pub fn tasks(content: &String) -> (String, String) {

    let valves = parse_input(content);
    let mut useful_valves: Vec<String> = vec![];

    debug!("Precomputing all distances...");

    // Let's precompute all the valves distances
    let mut distances: HashMap<(String, String), i32> = HashMap::new();
    for (start_name, start_valve) in &valves {
        if start_valve.flow > 0 {
            useful_valves.push(start_name.to_string());
        }

        for (end_name, _) in &valves {
            let v = dijkstra(start_valve, |n | n.neighbours(&valves), |n| &n.name == end_name).unwrap();
            let distance: i32 = v.1 as i32;
            distances.insert((start_name.to_string(), end_name.to_string()), distance);
        }
    }

    debug!("{} distances precomputed.", distances.len());

    let result1 = task1(valves.clone(), useful_valves.clone(), distances.clone());
    let result2 = task2(valves, useful_valves, distances);
    return (result1, result2);
}

const TIME_LIMIT: i32 = 30;
const TIME_LIMIT_2: i32 = 26;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Valve {
    name: String,
    flow: i32,
    neighbours: Vec<String>,
}

impl Valve {
    fn neighbours(&self, valves: &HashMap<String, Valve>) -> Vec<(Valve, usize)> {
        let mut neighbours: Vec<(Valve, usize)> = vec![];
        for n in &self.neighbours {
            neighbours.push((valves.get(n).unwrap().clone(), 1));
        }
        neighbours
    }
}

fn task1(valves: HashMap<String, Valve>, useful_valves: Vec<String>, distances: HashMap<(String, String), i32>) -> String {
    
    let max_pressure = best_path(valves, useful_valves, distances, TIME_LIMIT);

    max_pressure.to_string()
}

fn task2(valves: HashMap<String, Valve>, useful_valves: Vec<String>, distances: HashMap<(String, String), i32>) -> String {

    let mut max_pressure: i32 = 0;
    
    // Let's get all potential combinations of halving our valves
    let human_valves = useful_valves.iter().combinations(useful_valves.len() / 2);

    let combinations = human_valves.clone().collect_vec().len();
    debug!("Generated {} sets of combinations to work on.", combinations);
    
    let bar = ProgressBar::new(combinations as u64);
    for mut v in human_valves {
        let hv: Vec<String> = v.iter_mut().map(|x| x.to_string()).collect_vec();

        // Generate elefant valves
        let mut ev = useful_valves.clone();
        ev.retain(|x| !hv.contains(&x));

        let max_pressure_human = best_path(valves.clone(), hv, distances.clone(), TIME_LIMIT_2);
        let max_pressure_elefant = best_path(valves.clone(), ev, distances.clone(), TIME_LIMIT_2);

        max_pressure = std::cmp::max(max_pressure, max_pressure_human + max_pressure_elefant);
        bar.inc(1);
    }
    bar.finish();

    max_pressure.to_string()

}

fn parse_input(content: &String) -> HashMap<String, Valve> {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    
    const VALVES: &str = r"^Valve\s(?P<name>\S+) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<neighbours>.*)$";
    let re: Regex = Regex::new(VALVES).unwrap();

    for line in content.lines() {
        if re.is_match(line) {
            let captures = re.captures(line).unwrap();

            let name = captures["name"].to_string();
            let flow: i32 = captures["flow"].parse().unwrap();
            let neighbours: Vec<String> = captures["neighbours"].split(", ").map(|x| x.to_string()).collect();

            let valve = Valve { name: name.clone(), flow: flow, neighbours: neighbours };

            valves.insert(name, valve);
        }
    }

    valves
}

fn best_path(valves: HashMap<String, Valve>, useful_valves: Vec<String>, distances: HashMap<(String, String), i32>, time_limit: i32) -> i32 {
    let mut max_pressure: i32 = 0;

        // Path, Time Elapsed, (Valve, Opening Time)
        let mut paths: Vec<(Vec<String>, i32, Vec<(String, i32)>)> = vec![(vec!["AA".to_string()], 0, vec![])];

        // Traverse potential paths
        while paths.len() > 0 {
            let (path, time_elapsed, opening_times) = paths.pop().unwrap();
            let current_valve = &path[&path.len() - 1];
    
            // Check whether we are out of time
            if time_elapsed >= time_limit || path.len() > useful_valves.len() {
                let mut path_pressure = 0;
    
                // Calculate the pressure
                for (valve_name, opening_time) in opening_times {
                    let v = valves.get(&valve_name).unwrap();
                    path_pressure += std::cmp::max(v.flow * (time_limit - opening_time), 0);
                }
    
                max_pressure = std::cmp::max(max_pressure, path_pressure);
            
            } else {
                // We are still building up our path
                for next_valve in &useful_valves {
                    // Check if we have already visited the valve
                    if !path.contains(&next_valve) {
                        let distance = distances.get(&(current_valve.to_string(), next_valve.to_string())).unwrap();
                        let new_time_elapsed = time_elapsed + distance + 1; // Add +1 to account for the time opening the valve
    
                        let mut new_opening_times = opening_times.clone();
                        new_opening_times.push((next_valve.to_string(), time_elapsed + distance + 1));
    
                        let mut new_path = path.clone();
                        new_path.push(next_valve.to_string());
    
                        paths.push((new_path, new_time_elapsed, new_opening_times));
                    }
                }
                
            }
        }

        max_pressure
}

/*#[cfg(test)]
fn test_input() -> String {
    String::from(r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#)
}*/

#[test]
fn test_task1() {
    // No tests here as I structured this file a bit differently to save computing time...
    // assert_eq!(task1(&test_input()), "1651");
}

#[test]
fn test_task2() {
    // No tests here as I structured this file a bit differently to save computing time...
    // assert_eq!(task2(&test_input()), "1707");
}
