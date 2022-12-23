use std::{collections::HashMap, char::MAX};

use indicatif::ProgressBar;
use log::{debug, error};
use parse_display::{Display, FromStr};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let mut quality = 0;

    let blueprints = parse_input(content);
    //for blueprint in blueprints {
    //    quality += solve(blueprint) * blueprint.number;
    //}
    quality = solve(blueprints[0]) * blueprints[0].number;

    quality.to_string()
}

fn task2(content: &String) -> String {
    String::from("")
}

#[derive(Display, FromStr, Debug, Clone, Copy)]
#[display("Blueprint {number}: Each ore robot costs {cost_ore_robot_ore} ore. Each clay robot costs {cost_clay_robot_ore} ore. Each obsidian robot costs {cost_obsidian_robot_ore} ore and {cost_obsidian_robot_clay} clay. Each geode robot costs {cost_geode_robot_ore} ore and {cost_geode_robot_obsidian} obsidian.")]
struct Blueprint {
    number: i32,
    cost_ore_robot_ore: i32,
    cost_clay_robot_ore: i32,
    cost_obsidian_robot_ore: i32,
    cost_obsidian_robot_clay: i32,
    cost_geode_robot_ore: i32,
    cost_geode_robot_obsidian: i32
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    round: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    geode_robots: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geodes: i32,
}

fn solve(blueprint: Blueprint) -> i32 {
    let mut geodes = 0;
    const MAX_ROUNDS: i32 = 24;

    let mut builds: Vec<State> = vec![State{ round: 1, ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0, ore: 1, clay: 0, obsidian: 0, geodes: 0 }];
    let mut all_builds: HashMap<State, bool> = HashMap::new();
    let mut best_states: HashMap<i32, State> = HashMap::new();

    let bar = ProgressBar::new(builds.len() as u64);

    let mut i = 0;
    while builds.len() > 0 {
        if i == 10 {
            break;
        }
        i += 1;

        let build = builds.pop().unwrap();
        bar.inc(1);

        println!("{:?}", build);

        // We are done! Yay!
        if build.round == MAX_ROUNDS {
            //println!("We are at max rounds with geodes: {}", build.geodes);
            //println!("{:?}", build);
            geodes = std::cmp::max(geodes, build.geodes);
            continue;
        }

        // We can't find a better solution with this path for the current round, so we abort
        if !best_states.contains_key(&build.round) {
            best_states.insert(build.round.clone(), build);
        } else {
            let best = best_states.get(&build.round).unwrap();
            if build.geode_robots * (MAX_ROUNDS - build.round) + build.geodes < best.geode_robots * (MAX_ROUNDS - build.round) + best.geodes {
                println!("Not going down this rabbithole: {:?}", build);
                // Don't follow down this path / rabbithole
                continue;
            }
        }

        let mut robots_build = false;
        
        // Try to build a geode robot next as soon as possible
        if build.obsidian_robots > 0 {
            let rounds_until_build_geode = std::cmp::max((blueprint.cost_geode_robot_obsidian - build.obsidian) / build.obsidian_robots, std::cmp::max((blueprint.cost_geode_robot_ore - build.ore) / build.ore_robots, 1));
            let mut build_geode = build.clone();
    
            build_geode.ore = build.ore + build.ore_robots * rounds_until_build_geode;
            build_geode.clay = build.clay + build.clay_robots * rounds_until_build_geode;
            build_geode.obsidian = build.obsidian + build.obsidian_robots * rounds_until_build_geode;
            build_geode.geodes = build.geodes + build.geode_robots * rounds_until_build_geode;
    
            build_geode.ore -= blueprint.cost_geode_robot_ore;
            build_geode.obsidian -= blueprint.cost_geode_robot_obsidian;
            build_geode.geode_robots += 1;

            build_geode.round += rounds_until_build_geode;
    
            if !all_builds.contains_key(&build_geode) && build_geode.round <= MAX_ROUNDS {
                builds.push(build_geode);
                all_builds.insert(build_geode, true);
                bar.inc_length(1);
                robots_build = true;
            }
        }

        // Try to build an obsidian robot next as soon as possible
        if build.clay_robots > 0 && build.obsidian_robots < blueprint.cost_geode_robot_obsidian {
            let rounds_until_build_obsidian = std::cmp::max((blueprint.cost_obsidian_robot_clay - build.clay) / build.clay_robots, std::cmp::max((blueprint.cost_obsidian_robot_ore - build.ore) / build.ore_robots, 1));
            let mut build_obsidian = build.clone();
    
            build_obsidian.ore = build.ore + build.ore_robots * rounds_until_build_obsidian;
            build_obsidian.clay = build.clay + build.clay_robots * rounds_until_build_obsidian;
            build_obsidian.obsidian = build.obsidian + build.obsidian_robots * rounds_until_build_obsidian;
            build_obsidian.geodes = build.geodes + build.geode_robots * rounds_until_build_obsidian;
    
            build_obsidian.ore -= blueprint.cost_obsidian_robot_ore;
            build_obsidian.clay -= blueprint.cost_obsidian_robot_clay;
            build_obsidian.obsidian_robots += 1;

            build_obsidian.round += rounds_until_build_obsidian;
    
            if !all_builds.contains_key(&build_obsidian) && build_obsidian.round <= MAX_ROUNDS {
                builds.push(build_obsidian);
                all_builds.insert(build, true);
                bar.inc_length(1);
                robots_build = true;
            }
        }


        // Try to build a clay robot next as soon as possible
        if build.clay_robots < blueprint.cost_obsidian_robot_clay { // We don't need anymore if we are at full capacity
            let rounds_until_build_clay = std::cmp::max((blueprint.cost_clay_robot_ore - build.ore) / build.ore_robots, 1);
            let mut build_clay = build.clone();

            build_clay.ore = build.ore + build.ore_robots * rounds_until_build_clay;
            build_clay.clay = build.clay + build.clay_robots * rounds_until_build_clay;
            build_clay.obsidian = build.obsidian + build.obsidian_robots * rounds_until_build_clay;
            build_clay.geodes = build.geodes + build.geode_robots * rounds_until_build_clay;

            build_clay.ore -= blueprint.cost_clay_robot_ore;
            build_clay.clay_robots += 1;

            build_clay.round += rounds_until_build_clay;

            if !all_builds.contains_key(&build_clay) && build_clay.round <= MAX_ROUNDS {
                builds.push(build_clay);
                all_builds.insert(build_clay, true);
                bar.inc_length(1);
                robots_build = true;
            }
        }

        // Try to build an ore robot next as soon as possible
        if build.ore_robots < blueprint.cost_clay_robot_ore && build.ore_robots < blueprint.cost_geode_robot_ore && build.ore_robots < blueprint.cost_obsidian_robot_ore { // We don't need anymore if we are at full capacity
            let rounds_until_build_ore = std::cmp::max((blueprint.cost_ore_robot_ore - build.ore) / build.ore_robots, 1);
            let mut build_ore = build.clone();

            build_ore.ore = build.ore + build.ore_robots * rounds_until_build_ore;
            build_ore.clay = build.clay + build.clay_robots * rounds_until_build_ore;
            build_ore.obsidian = build.obsidian + build.obsidian_robots * rounds_until_build_ore;
            build_ore.geodes = build.geodes + build.geode_robots * rounds_until_build_ore;

            build_ore.ore -= blueprint.cost_ore_robot_ore;
            build_ore.ore_robots += 1;

            build_ore.round += rounds_until_build_ore;

            if !all_builds.contains_key(&build_ore) && build_ore.round <= MAX_ROUNDS {
                builds.push(build_ore);
                all_builds.insert(build_ore, true);
                bar.inc_length(1);
                robots_build = true;
            }
        }

        // Let's finish from this state when there can't be any robot build anymore
        if !robots_build {
            let rounds_until_finish = MAX_ROUNDS - build.round;
            let mut build_none = build.clone();
    
            build_none.ore = build.ore + build.ore_robots * rounds_until_finish;
            build_none.clay = build.clay + build.clay_robots * rounds_until_finish;
            build_none.obsidian = build.obsidian + build.obsidian_robots * rounds_until_finish;
            build_none.geodes = build.geodes + build.geode_robots * rounds_until_finish;
    
            builds.push(build_none);
            all_builds.insert(build_none, true);
            bar.inc_length(1);
        }


    }
    bar.finish();

    geodes
}

fn parse_input(content: &String) -> Vec<Blueprint> {
    let mut blueprints: Vec<Blueprint> = vec![];
    for line in content.lines() {
        let blueprint: Blueprint = line.parse().unwrap();
        blueprints.push(blueprint);
    }
    blueprints
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#)
}

#[test]
fn test_task1() {
    //assert_eq!(task1(&test_input()), "35");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "");
}
