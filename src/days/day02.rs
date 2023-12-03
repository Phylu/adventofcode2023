use regex::Regex;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let vec = prepare_input(content);
    let result: i32 = vec.iter().sum();

    return result.to_string();
}

fn task2(content: &String) -> String {
//    let vec = prepare_input_2(content);
//    let result: i32 = vec.iter().sum();

//    return result.to_string();
    return "".to_string();
}

fn prepare_input(content: &String) -> Vec<i32> {

    let mut vec = Vec::new();

    let max_red = 12;
    let max_greens = 13;
    let max_blue = 14;

    for line in content.lines() {

        if line != "" {
            let game_re = Regex::new(r"Game (?P<game>\d+):").unwrap();
            let game_captures = game_re.captures(line).unwrap();
            let game_number = String::from(&game_captures["game"]);

            let blues_re = Regex::new(r"(?P<blue>\d+) blue").unwrap();
            let blue_captures = blues_re.captures_iter(line);
            blue_captures.map(|capture| {
                let blues: i32 = capture.name("blue").unwrap().as_str().parse().unwrap();
                if blues > max_blue {
                    continue;
                }
            } );

            println!("{}", game_number);
            vec.push(format!("{}", game_number).parse().unwrap());
            
        }
    }
    
    return vec;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/2.txt").unwrap(); 
    assert_eq!(task1(&content), "8");
}

/*
#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/2.txt").unwrap(); 
    assert_eq!(task2(&content), "281");
}
*/