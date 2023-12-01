
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

    for line in content.lines() {
        if line != "" {
            let chars = line.chars();
            let mut current: Vec<u32> = Vec::new();

            // Forward Search
            for character in chars.clone() {
                let digit : Option<u32> = character.to_digit(10);
                if digit.is_some() {
                    current.push(digit.unwrap());
                    break;
                }
            }
            
            for character in chars.clone().rev() {
                let digit : Option<u32> = character.to_digit(10);
                if digit.is_some() {
                    current.push(digit.unwrap());
                    break;
                }
            }

            println!("{}{}", current[0], current[1]);
            vec.push(format!("{}{}", current[0], current[1]).parse().unwrap());
        }
    }
    
    return vec;
}

fn prepare_input_2(content: &String) -> Vec<i32> {

    let mut vec = Vec::new();

    for line in content.lines() {
        if line != "" {

            // Custom replaces some where we could have issues
            let line = line.replace("nineight", "9ight");
            let line = line.replace("eightwo", "8wo");
            let line = line.replace("eighthree", "8hree");
            let line = line.replace("twone", "2ne");

            let line = line.replace("one", "1");
            let line = line.replace("two", "2");
            let line = line.replace("three", "3");
            let line = line.replace("four", "4");
            let line = line.replace("five", "5");
            let line = line.replace("six", "6");
            let line = line.replace("seven", "7");
            let line = line.replace("eight", "8");
            let line = line.replace("nine", "9");

            println!("{}", line);

            let chars = line.chars();

            let mut current: Vec<u32> = Vec::new();

            // Forward Search
            for character in chars.clone() {
                let digit : Option<u32> = character.to_digit(10);
                if digit.is_some() {
                    current.push(digit.unwrap());
                    break;
                }
            }
            
            for character in chars.clone().rev() {
                let digit : Option<u32> = character.to_digit(10);
                if digit.is_some() {
                    current.push(digit.unwrap());
                    break;
                }
            }

            println!("{}{}", current[0], current[1]);
            vec.push(format!("{}{}", current[0], current[1]).parse().unwrap());
        }
    }
    
    return vec;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/1_1.txt").unwrap(); 
    assert_eq!(task1(&content), "142");
}


#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/1_2.txt").unwrap(); 
    assert_eq!(task2(&content), "281");
}
