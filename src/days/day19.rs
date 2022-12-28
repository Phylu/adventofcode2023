use indicatif::ProgressBar;
use log::trace;
use parse_display::{Display, FromStr};
use std::cmp::max;
use std::collections::{HashMap, HashSet};

const MAX_ROUNDS: i32 = 24;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let mut quality = 0;

    let blueprints = parse_input(content);
    for blueprint in blueprints {
        let res = solve(blueprint);
        println!("Blueprint {} max geodes is {}", blueprint.number, res);
        quality += res * blueprint.number;
    }

    quality.to_string()
}

fn task2(content: &String) -> String {
    String::from("")
}

#[derive(Debug, Display, PartialEq, Eq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
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
    cost_geode_robot_obsidian: i32,
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

impl State {
    fn abort(self, best: State) -> bool {
        false
        //self.geode_robots * (MAX_ROUNDS - self.round) + self.geodes
        //    < best.geode_robots * (MAX_ROUNDS - self.round) + best.geodes
    }

    fn build_robot(self, blueprint: Blueprint, mat: Material) -> State {
        let mut new_state = self.clone();

        let mut dore = 0;
        let mut dclay = 0;
        let mut dobsidian = 0;

        match mat {
            Material::Ore => {
                dore = blueprint.cost_ore_robot_ore;
                new_state.ore_robots += 1;
            }
            Material::Clay => {
                dore = blueprint.cost_clay_robot_ore;
                new_state.clay_robots += 1;
            }
            Material::Obsidian => {
                dore = blueprint.cost_obsidian_robot_ore;
                dclay = blueprint.cost_obsidian_robot_clay;
                new_state.obsidian_robots += 1;
            }
            Material::Geode => {
                dore = blueprint.cost_geode_robot_ore;
                dobsidian = blueprint.cost_geode_robot_obsidian;
                new_state.geode_robots += 1;
            }
        }

        let ore_rounds = {
            let a = dore - self.ore;
            let b = self.ore_robots;
            (a + b - 1) / b
        };
        let clay_rounds = if dclay == 0 {
            0
        } else {
            let a = dclay - self.clay;
            let b = self.clay_robots;
            (a + b - 1) / b
        };
        let obsidian_rounds = if dobsidian == 0 {
            0
        } else {
            let a = dobsidian - self.obsidian;
            let b = self.obsidian_robots;
            (a + b - 1) / b
        };

        // We need to wait for the material to gather and then one extra round for building the robot
        let rounds = max(ore_rounds, max(clay_rounds, max(obsidian_rounds, 0))) + 1;

        new_state.round += rounds;

        //println!("{:?}", self);
        //println!("Building: {}", mat);
        //println!("Rounds until ore is ready: {}", ore_rounds);
        //println!("Rounds until clay is ready: {}", clay_rounds);
        //println!("Rounds until obsidian is ready: {}", obsidian_rounds);
        //println!("Robot will be ready in round: {}", new_state.round);

        new_state.ore = self.ore + (rounds * self.ore_robots) - dore;
        new_state.clay = self.clay + (rounds * self.clay_robots) - dclay;
        new_state.obsidian = self.obsidian + (rounds * self.obsidian_robots) - dobsidian;
        new_state.geodes = self.geodes + (rounds * self.geode_robots);

        new_state
    }
}

fn solve(blueprint: Blueprint) -> i32 {
    let mut geodes = 0;

    let mut builds: Vec<State> = vec![State {
        round: 1,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        ore: 1,
        clay: 0,
        obsidian: 0,
        geodes: 0,
    }];
    let mut all_builds: HashSet<State> = HashSet::new();
    let mut best_states: HashMap<i32, State> = HashMap::new();

    let bar = ProgressBar::new(builds.len() as u64);

    while builds.len() > 0 {

        let build = builds.pop().unwrap();
        bar.inc(1);

        // We are done! Yay!
        if build.round == MAX_ROUNDS {
            trace!("We are at max rounds with geodes: {}", build.geodes);
            trace!("{:?}", build);
            geodes = std::cmp::max(geodes, build.geodes);
            continue;
        }

        // We can't find a better solution with this path for the current round, so we abort
        if !best_states.contains_key(&build.round) {
            best_states.insert(build.round.clone(), build);
        } else {
            let best = best_states.get(&build.round).unwrap();
            if build.abort(*best) {
                trace!("Not going down this rabbithole: {:?}", build);
                // Don't follow down this path / rabbithole
                continue;
            }
        }

        let mut robots_build = false;

        // Try to build a geode robot next as soon as possible
        if build.obsidian_robots > 0 {
            let build_geode = build.build_robot(blueprint, Material::Geode);

            if !all_builds.contains(&build_geode) && build_geode.round < MAX_ROUNDS {
                builds.push(build_geode);
                all_builds.insert(build_geode);
                bar.inc_length(1);
                robots_build = true;
            }
        }

        // Try to build an obsidian robot next as soon as possible
        if build.clay_robots > 0 && build.obsidian_robots < blueprint.cost_geode_robot_obsidian {
            let build_obsidian = build.build_robot(blueprint, Material::Obsidian);

            if !all_builds.contains(&build_obsidian) && build_obsidian.round < MAX_ROUNDS {
                builds.push(build_obsidian);
                all_builds.insert(build);
                bar.inc_length(1);
                robots_build = true;
            }
        }

        // Try to build a clay robot next as soon as possible
        // We don't need anymore if we are at full capacity
        if build.clay_robots < blueprint.cost_obsidian_robot_clay {
            let build_clay = build.build_robot(blueprint, Material::Clay);

            if !all_builds.contains(&build_clay) && build_clay.round < MAX_ROUNDS {
                builds.push(build_clay);
                all_builds.insert(build_clay);
                bar.inc_length(1);
                robots_build = true;
            }
        }

        // Try to build an ore robot next as soon as possible
        // We don't need anymore if we are at full capacity
        if build.ore_robots < blueprint.cost_clay_robot_ore
            && build.ore_robots < blueprint.cost_geode_robot_ore
            && build.ore_robots < blueprint.cost_obsidian_robot_ore
        {
            let build_ore = build.build_robot(blueprint, Material::Ore);

            if !all_builds.contains(&build_ore) && build_ore.round < MAX_ROUNDS {
                builds.push(build_ore);
                all_builds.insert(build_ore);
                bar.inc_length(1);
                robots_build = true;
            }
        }

        // Let's finish from this state when there can't be any robot build anymore
        //if !robots_build {
            let rounds_until_finish = MAX_ROUNDS - build.round;
            let mut build_none = build.clone();

            build_none.round += rounds_until_finish;
            build_none.ore = build.ore + build.ore_robots * rounds_until_finish;
            build_none.clay = build.clay + build.clay_robots * rounds_until_finish;
            build_none.obsidian = build.obsidian + build.obsidian_robots * rounds_until_finish;
            build_none.geodes = build.geodes + build.geode_robots * rounds_until_finish;

            builds.push(build_none);
            all_builds.insert(build_none);
            bar.inc_length(1);
        //}
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
    String::from(
        r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#,
    )
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "34");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "");
}
