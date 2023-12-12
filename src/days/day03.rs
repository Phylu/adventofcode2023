/*
use pathfinding::num_traits::Num;

struct Number {
    number: String,
    row: i32,
    col: i32,
}

impl Number {
    fn length(&self) -> usize {
        return self.number.chars().count();
    }

    fn append(&self, char: char) -> Number {
        return Number { number: self.number.clone(), row: self.row, col: self.col }
    }
}

struct Symbol {
    row: i32,
    col: i32,
}

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
    /*let vec = prepare_input_2(content);
    let result: i32 = vec.iter().sum();

   return result.to_string();*/
   return "".to_string();
}

fn prepare_input_1(content: &String) -> Vec<i32> {

    let mut vec = Vec::new();
    let mut row = 0;
    let mut numbers: Vec<Number> = Vec::new();
    // let mut symbols = [];

    for line in content.lines() {
        let mut col = 0;

        for c in line.chars() {

        }

        row += 1;
    }


    return vec;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/3.txt").unwrap(); 
    assert_eq!(task1(&content), "4361");
}

#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/2.txt").unwrap(); 
    assert_eq!(task2(&content), "2286");
}
*/
