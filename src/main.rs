use std::process;
use clap::Parser;
use clap_verbosity_flag::{Verbosity, InfoLevel};
mod days;

#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
    day: i32,
}

fn empty(_content: &String) -> (String, String) {
    println!("This day has not yet been implemented!");
    process::exit(1);
}

fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    let content = std::fs::read_to_string(format!("input/{}.txt", &args.day)).unwrap(); 

    let tasks = match &args.day {
        &1 => days::day01::tasks,
        /*&2 => days::day02::tasks,
        &3 => days::day03::tasks,
        &4 => days::day04::tasks,
        &5 => days::day05::tasks,
        &6 => days::day06::tasks,
        &7 => days::day07::tasks,
        &8 => days::day08::tasks,
        &9 => days::day09::tasks,
        &10 => days::day10::tasks,
        &11 => days::day11::tasks,
        &12 => days::day12::tasks,
        &13 => days::day13::tasks,
        &14 => days::day14::tasks,
        &15 => days::day15::tasks,
        &16 => days::day16::tasks,
        &17 => days::day17::tasks,
        &18 => days::day18::tasks,
        &19 => days::day19::tasks,
        &20 => days::day20::tasks,
        &21 => days::day21::tasks,
        &22 => days::day22::tasks,
        &23 => days::day23::tasks,
        &24 => days::day24::tasks,
        &25 => days::day25::tasks,*/
        _ => empty,
    };

    let (result1, result2) = tasks(&content);
    println!("The results for day {} are:", &args.day);
    println!("Task 1: {}", result1);
    println!("Task 2: {}", result2);
}
