pub fn tasks(content: &String) {
    task1(content);
    task2(content);
}

fn task1(content: &String) {

    let mut points = 0;
    
    for line in content.lines() {

        let split: Vec<&str> = line.split(",").collect();

        let first: Vec<&str> = split[0].split("-").collect();
        let second: Vec<&str> = split[1].split("-").collect();
        
        let first_start : i32 = first[0].parse().unwrap();
        let first_end : i32 = first[1].parse().unwrap();
        let second_start : i32 = second[0].parse().unwrap();
        let second_end : i32 = second[1].parse().unwrap();

        if (first_start <= second_start && first_end >= second_end) || (second_start <= first_start && second_end >= first_end) {
            points += 1;
        }

    }

    println!("{}", points);
}

fn task2(content: &String) {

    let mut points = 0;

    for line in content.lines() {

        let split: Vec<&str> = line.split(",").collect();

        let first: Vec<&str> = split[0].split("-").collect();
        let second: Vec<&str> = split[1].split("-").collect();
        
        let first_start : i32 = first[0].parse().unwrap();
        let first_end : i32 = first[1].parse().unwrap();
        let second_start : i32 = second[0].parse().unwrap();
        let second_end : i32 = second[1].parse().unwrap();

        if (first_start <= second_start && second_start <= first_end) || (second_start <= first_start && first_start <= second_end) {
            points += 1;
        }
    }

    println!("{}", points);
}
