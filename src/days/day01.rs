
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

            // println!("{}{}", current[0], current[1]);
            vec.push(format!("{}{}", current[0], current[1]).parse().unwrap());
        }
    }
    
    return vec;
}

fn prepare_input_2(content: &String) -> Vec<i32> {

    let mut vec = Vec::new();

    for line in content.lines() {
        if line != "" {

            // Duplicate the number string and place the literal digit in between
            // in order to keep letters that appear in multiple numbers
            // E.g. threeightwo = 382
            let line = line.replace("one", "one1one");
            let line = line.replace("two", "two2two");
            let line = line.replace("three", "three3three");
            let line = line.replace("four", "four4four");
            let line = line.replace("five", "five5five");
            let line = line.replace("six", "six6six");
            let line = line.replace("seven", "seven7seven");
            let line = line.replace("eight", "eight8eight");
            let line = line.replace("nine", "nine9nine");

            // println!("{}", line);

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

            // println!("{}{}", current[0], current[1]);
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
