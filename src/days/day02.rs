use regex::Regex;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let vec = prepare_input_1(content);
    let result: i32 = vec.iter().sum();

    return result.to_string();
}

fn task2(content: &String) -> String {
    let vec = prepare_input_2(content);
    let result: i32 = vec.iter().sum();

   return result.to_string();
}

fn prepare_input_1(content: &String) -> Vec<i32> {

    let mut vec = Vec::new();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    'lines: for line in content.lines() {

        if line != "" {
            let game_re = Regex::new(r"Game (?P<game>\d+):").unwrap();
            let game_captures = game_re.captures(line).unwrap();
            let game_number = String::from(&game_captures["game"]);

            let reds_re = Regex::new(r"(?P<red>\d+) red").unwrap();
            for reds in reds_re.captures_iter(line) {
                let int_reds: i32 = reds["red"].parse().unwrap();
                if int_reds > max_red {
                    continue 'lines;
                }
            }

            let greens_re = Regex::new(r"(?P<green>\d+) green").unwrap();
            for greens in greens_re.captures_iter(line) {
                let int_greens: i32 = greens["green"].parse().unwrap();
                if int_greens > max_green {
                    continue 'lines;
                }
            }

            let blues_re = Regex::new(r"(?P<blue>\d+) blue").unwrap();
            for blues in blues_re.captures_iter(line) {
                let int_blues: i32 = blues["blue"].parse().unwrap();
                if int_blues > max_blue {
                    continue 'lines;
                }
            }

            vec.push(format!("{}", game_number).parse().unwrap());
            
        }
    }
    
    return vec;
}

fn prepare_input_2(content: &String) -> Vec<i32> {

    let mut vec = Vec::new();

    for line in content.lines() {

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        if line != "" {
            
            let reds_re = Regex::new(r"(?P<red>\d+) red").unwrap();
            for reds in reds_re.captures_iter(line) {
                let int_reds: i32 = reds["red"].parse().unwrap();
                if int_reds > max_red {
                    max_red = int_reds;
                }
            }

            let greens_re = Regex::new(r"(?P<green>\d+) green").unwrap();
            for greens in greens_re.captures_iter(line) {
                let int_greens: i32 = greens["green"].parse().unwrap();
                if int_greens > max_green {
                    max_green = int_greens;
                }
            }

            let blues_re = Regex::new(r"(?P<blue>\d+) blue").unwrap();
            for blues in blues_re.captures_iter(line) {
                let int_blues: i32 = blues["blue"].parse().unwrap();
                if int_blues > max_blue {
                    max_blue = int_blues
                }
            }

            vec.push(format!("{}", max_red * max_green * max_blue).parse().unwrap());
            
        }
    }
    
    return vec;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/2.txt").unwrap(); 
    assert_eq!(task1(&content), "8");
}


#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/2.txt").unwrap(); 
    assert_eq!(task2(&content), "2286");
}
