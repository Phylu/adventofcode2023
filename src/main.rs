use clap::Parser;
mod days;

#[derive(Parser)]
struct Cli {
    day: i32,
}

fn empty(_content: &String){
    println!("This day has not yet been implemented!");
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(format!("input/{}.txt", &args.day)).unwrap(); 

    let tasks = match &args.day {
        &1 => days::day1::tasks,
        &2 => days::day2::tasks,
        _ => empty,
    };

    tasks(&content);
}
