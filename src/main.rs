
use std::process;
use clap::Parser;
mod days;

#[derive(Parser)]
struct Cli {
    day: i32,
}

fn empty(_content: &String) -> (String, String) {
    println!("This day has not yet been implemented!");
    process::exit(1);
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(format!("input/{}.txt", &args.day)).unwrap(); 

    let tasks = match &args.day {
        &1 => days::day1::tasks,
        &2 => days::day2::tasks,
        &3 => days::day3::tasks,
        &4 => days::day4::tasks,
        &5 => days::day5::tasks,
        _ => empty,
    };

    let (result1, result2) = tasks(&content);
    println!("The results for day {} are:", &args.day);
    println!("Task 1: {}", result1);
    println!("Task 2: {}", result2);
}
