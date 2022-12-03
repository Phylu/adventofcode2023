use itertools::Itertools;

pub fn tasks(content: &String) {
    task1(content);
    task2(content);
}

fn task1(content: &String) {

    let mut points = 0;
    
    for line in content.lines() {

        let (split1, split2) = line.split_at(line.len() / 2);

        let sorted1 = split1.chars().sorted();
        let sorted2 = split2.chars().sorted();

        let slice1 = sorted1.as_slice();
        let slice2 = sorted2.as_slice();

        let mut duplicate = &' ';

        for s1 in slice1 {

            for s2 in slice2 {
                if s1 == s2 {
                    duplicate = s1;
                    break;
                }
            
            }

            if duplicate != &' ' {
                break;
            }
        }

        let mut uppercase_points = 0;
        if duplicate.is_ascii_uppercase() {
            uppercase_points = 26;
        }
        
        let d1 = &duplicate.to_lowercase().next().unwrap();
        
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

        points += current_points + uppercase_points;

    }

    println!("{}", points);
}

fn task2(content: &String) {

    let points = 0;
    println!("{}", points);
}
