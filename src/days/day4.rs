pub fn tasks(content: &String) -> (i32, i32) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> i32 {

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

    return points;
}

fn task2(content: &String) -> i32 {

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

    return points;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input/4.txt").unwrap(); 
    assert_eq!(task1(&content), 433);
}

#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input/4.txt").unwrap(); 
    assert_eq!(task2(&content), 852);
}
