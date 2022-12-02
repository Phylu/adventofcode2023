pub fn tasks(content: &String) {
    task1(content);
    task2(content);
}

pub fn task1(content: &String) {

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

pub fn task2(content: &String) {

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
