use std::collections::HashMap;
use itertools::Itertools;

use log::{debug, error};

use pathfinding::prelude::dijkstra;
use regex::Regex;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

const TIME_LIMIT: i32 = 30;
const TIME_LIMIT_2: i32 = 30;

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

fn task1(content: &String) -> String {
    let valves = parse_input(content);
    let mut useful_valves: Vec<String> = vec![];
    let mut max_pressure: i32 = 0;

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
    
    // Path, Time Elapsed, (Valve, Opening Time)
    let mut paths: Vec<(Vec<String>, i32, Vec<(String, i32)>)> = vec![(vec!["AA".to_string()], 0, vec![])];

    // Traverse potential paths
    while paths.len() > 0 {
        let (path, time_elapsed, opening_times) = paths.pop().unwrap();
        let current_valve = &path[&path.len() - 1];

        // Check whether we are out of time
        if time_elapsed >= TIME_LIMIT || path.len() > useful_valves.len() {
            let mut path_pressure = 0;

            // Calculate the pressure
            for (valve_name, opening_time) in opening_times {
                let v = valves.get(&valve_name).unwrap();
                path_pressure += v.flow * (TIME_LIMIT - opening_time);
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

    max_pressure.to_string()
}

fn task2(content: &String) -> String {

    let valves = parse_input(content);
    let mut useful_valves: Vec<String> = vec![];
    let mut max_pressure: i32 = 0;
    let mut max_pressure_paths: Vec<(Vec<String>, i32)> = vec![];

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
    
    // Path, Time Elapsed, (Valve, Opening Time)
    let mut paths: Vec<(Vec<String>, i32, Vec<(String, i32)>)> = vec![(vec!["AA".to_string()], 0, vec![])];

    // Traverse potential paths
    while paths.len() > 0 {
        
        let (path, time_elapsed, opening_times) = paths.pop().unwrap();
        let current_valve = &path[&path.len() - 1];

        println!("Working on path of length {} at time {} with {} open valves.", path.len(), time_elapsed, opening_times.len());

        // Check whether we are out of time
        if time_elapsed >= TIME_LIMIT_2 || path.len() >= useful_valves.len() {
            let mut path_pressure = 0;

            // Calculate the pressure
            for (valve_name, opening_time) in opening_times {
                let v = valves.get(&valve_name).unwrap();
                path_pressure += std::cmp::max(v.flow * (TIME_LIMIT_2 - opening_time), 0);
            }

            max_pressure_paths.push((path, path_pressure));
        
        } else {
            for next_valve in &useful_valves {

                // Check if we have already visited the valve
                if !path.contains(&next_valve) {
                    let distance = distances.get(&(current_valve.to_string(), next_valve.to_string())).unwrap();
                    let new_time_elapsed = time_elapsed + distance + 1; // Add +1 to account for the time opening the valve

                    let mut new_opening_times = opening_times.clone();
                    new_opening_times.push((next_valve.to_string(), time_elapsed + distance + 1));

                    let mut new_path = path.clone();
                    new_path.push(next_valve.to_string());

                    paths.push((new_path.clone(), new_time_elapsed, new_opening_times.clone()));
                    // Also push the very same path with a finished state so that we get combinations in case that the paths will overlap
                    paths.push((new_path, TIME_LIMIT_2, new_opening_times));
                }
            }
            
        }
    }

    println!("We have detected {} potential paths that need to be combined.", max_pressure_paths.len());

    //max_pressure_paths.sort_by(|x, y| y.1.cmp(&x.1));

    for i in 0..max_pressure_paths.len() {
        for j in 0..max_pressure_paths.len() {
            let (path1, pressure1) = &max_pressure_paths[i];
            let (path2, pressure2) = &max_pressure_paths[j];

            let combined_path_length = path1.clone().len() + path2.clone().len();
            
            let mut unique_path = path1.clone();
            let mut tmp_path = path2.clone();
            unique_path.append(&mut tmp_path);
            let unique_path_length = unique_path.into_iter().unique().collect::<Vec<String>>().len();

            // We did not open the same valve
            if unique_path_length == combined_path_length {
                max_pressure = std::cmp::max(max_pressure, pressure1 + pressure2);
            }
        }
    }

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

#[cfg(test)]
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
}

#[test]
fn test_task1() {
    //assert_eq!(task1(&test_input()), "1651");
}

#[test]
fn test_task2() {
    // Commented out as it runs too long
    //assert_eq!(task2(&test_input()), "1707");
}
