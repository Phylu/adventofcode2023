use std::fmt;
use log::{debug, trace};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let mut numbers = parse_input(content);
    let mut i: i64 = 0;

    debug!("Initial arrangement:");
    debug!("{:?}", numbers);
    
    while i < numbers.len() as i64 {
        
        // Only move the number if it has not been moved.
        if !numbers[i as usize].moved {

            let mut current = numbers.remove(i as usize);    
            let new_position = (i + current.number).rem_euclid(numbers.len() as i64);            
            current.moved = true;

            if new_position == 0 && current.number.is_negative() {
                numbers.push(current);
            } else {
                numbers.insert(new_position as usize, current);
            }
            
            let modulo: i64 = numbers.len() as i64;
            let current_number = numbers[new_position as usize].number;
            let previous_number = numbers[(new_position - 1).rem_euclid(modulo) as usize].number;
            let next_number = numbers[(new_position + 1).rem_euclid(modulo) as usize].number;

            debug!("{} moves between {} and {}:", current_number, previous_number, next_number);
            trace!("{:?}", numbers);

            // For simplicity, we always check the current index again to ensure
            // Otherwise we would need to check for overflows etc...
            i -= 1;
        }

        i += 1;
    }

    get_result(numbers).to_string()
}

fn task2(content: &String) -> String {
    let mut numbers = parse_input_2(content);
    
    debug!("Initial arrangement:");
    debug!("{:?}", numbers);
    
    for round in 0..10 {

        for i in 0..(numbers.len() + 1) as i64 {

            for j in 0..numbers.len() as i64 {

                if numbers[j as usize].position == i as i32 {

                    debug!("{:?}", numbers);

                    let current = numbers.remove(j as usize);
                    let mut new_position = (j + current.number).rem_euclid(numbers.len() as i64);
                    if new_position == 0 && current.number.is_negative() {
                        new_position = numbers.len() as i64;
                    }

                    debug!("Moving number {} at position {} to new position {} because initial position was {}.", current.number, j, new_position, i);        

                    numbers.insert(new_position as usize, current);
                    
                    let modulo: i64 = numbers.len() as i64;
                    let current_number = numbers[new_position as usize].number;
                    let previous_number = numbers[(new_position - 1).rem_euclid(modulo) as usize].number;
                    let next_number = numbers[(new_position + 1).rem_euclid(modulo) as usize].number;
        
                    debug!("{} moves between {} and {}.\n", current_number, previous_number, next_number);
                    trace!("{:?}", numbers);

                    break
                }
            }
        }

        debug!("After {} rounds of mixing: ", round + 1);
        debug!("{:?}", numbers);

    }

    get_result(numbers).to_string()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Enumber {
    number: i64,
    moved: bool,
    position: i32,
}

impl fmt::Display for Enumber {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.number)
    }
}

fn get_result(numbers: Vec<Enumber>) -> i64 {
    let mut zero_pos = 0;
    for i in 0..numbers.len() {
        if numbers[i].number == 0 {
            zero_pos = i;
            break;
        }
    }

    let first_pos = (zero_pos + 1000) % numbers.len();
    let second_pos = (zero_pos + 2000) % numbers.len();
    let third_pos = (zero_pos + 3000) % numbers.len();

    numbers[first_pos].number + numbers[second_pos].number + numbers[third_pos].number
}

fn parse_input(content: &String) -> Vec<Enumber> {
    let mut numbers: Vec<Enumber> = vec![];
    for line in content.lines() {
        if line.len() > 0 {
            let number: i64 = line.parse().unwrap();
            let enumber = Enumber {
                number: number,
                moved: false,
                position: 0
            };
            numbers.push(enumber);
        }
    }

    numbers
}

fn parse_input_2(content: &String) -> Vec<Enumber> {
    let mut numbers: Vec<Enumber> = vec![];
    let mut i = 0;
    for line in content.lines() {
        if line.len() > 0 {
            let number: i64 = line.parse().unwrap();
            let enumber = Enumber {
                number: number * 811589153,
                moved: false,
                position: i,
            };
            numbers.push(enumber);
        }
        i += 1;
    }

    numbers
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"1
2
-3
3
-2
0
4
"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "3");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "1623178306");
}
