use clap::Parser;

#[derive(Parser)]
struct Cli {
    day: i32,
}

fn d1_p1(content: &String) {

    let mut max = 0;
    let mut current = 0;

    for line in content.lines() {
        if line == "" {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let number : i32 = line.parse().unwrap();
            current += number;
        }
    }

    if current > max {
        max = current;
    }  

    println!("{}", max);

}

fn d1_p2(content: &String) {

    let mut vec = Vec::new();
    let mut current = 0;

    for line in content.lines() {
        if line == "" {
            vec.push(current);
            current = 0;
        } else {
            let number : i32 = line.parse().unwrap();
            current += number;
        }
    }
    vec.push(current);
    vec.sort();

    let top3 = &vec[vec.len()-3..vec.len()];
    let sum: i32 = top3.iter().sum();

    println!("{}", sum)

}

fn d2_p1(content: &String) {

    let mut points = 0;
    let mut game: Vec<&str>;

    // A X Rock
    // B Y Paper
    // C Z Scissors

    for line in content.lines() {
        
        game = line.split(" ").collect();

       // Points for own fist
        let fist_points = match game[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
 
        // Points for game result
        let game_points = match &*game {
            ["A", "Y"] | ["B", "Z"] | ["C", "X"] => 6,
            ["A", "X"] | ["B", "Y"] | ["C", "Z"] => 3,
            _ => 0
      };

        points += fist_points + game_points;
       }
    
    println!("{}", points);
}

fn d2_p2(content: &String) {

    let mut points = 0;
    let mut game: Vec<&str>;

    // A Rock
    // B Paper
    // C Scissors

    for line in content.lines() {
        
        game = line.split(" ").collect();

       // Points for own fist
        let game_points = match game[1] {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        };
 
        // Points for game result
        let fist_points = match &*game {
            ["A", "Y"] | ["B", "X"] | ["C", "Z"] => 1,
            ["A", "Z"] | ["B", "Y"] | ["C", "X"] => 2,
            _ => 3
      };

        points += fist_points + game_points;
       }
    
    println!("{}", points);
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(format!("input/{}.txt", &args.day)).unwrap(); 

    if &args.day == &1 {
        d1_p1(&content);
        d1_p2(&content);
    }

    if &args.day == &2 {
        d2_p1(&content);
        d2_p2(&content);
    }
}
