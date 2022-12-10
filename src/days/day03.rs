use itertools::Itertools;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String)  -> String {

    let mut points = 0;
    
    for line in content.lines() {
        let (split1, split2) = line.split_at(line.len() / 2);
        let dup = duplicate(split1, split2);
        points += calculate_points(dup);
    }

    return points.to_string();
}

fn task2(content: &String) -> String {

    let mut points = 0;
    let mut team = vec![];

    for line in content.lines() {
        
        team.push(line);

        if team.len() == 3 {
            let dup = duplicate3(team[0], team[1], team[2]);
            points += calculate_points(dup);

            team = vec![] // Reset the team after 3 elves
        }

    }

    return points.to_string();
}

fn duplicate(split1: &str, split2: &str) -> char {

    let sorted1 = split1.chars().sorted();
    let sorted2 = split2.chars().sorted();

    let slice1 = sorted1.as_slice();
    let slice2 = sorted2.as_slice();

    for s1 in slice1 {

        for s2 in slice2 {
            if s1 == s2 {
                return *s1;
            }
        
        }
    }

    return ' ';

}

fn duplicate3(split1: &str, split2: &str, split3: &str) -> char {

    let sorted1 = split1.chars().sorted();
    let sorted2 = split2.chars().sorted();
    let sorted3 = split3.chars().sorted();

    let slice1 = sorted1.as_slice();
    let slice2 = sorted2.as_slice();
    let slice3 = sorted3.as_slice();

    for s1 in slice1 {

        for s2 in slice2 {

            if s1 == s2 {

                for s3 in slice3 {
                    if s1 == s3 {
                        return *s3;
                    }
                }
            }
        
        }
    }

    return ' ';

}

fn calculate_points(character: char) -> i32 {
    let mut uppercase_points = 0;
    if character.is_ascii_uppercase() {
        uppercase_points = 26;
    }
    
    let d1 = character.to_lowercase().next().unwrap();
    
    let current_points = match d1 {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        _ => 0,
    };

    return uppercase_points + current_points;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input/3.txt").unwrap(); 
    assert_eq!(task1(&content), "7727");
}

#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input/3.txt").unwrap(); 
    assert_eq!(task2(&content), "2609");
}
