use itertools::Itertools;
use log::{debug, error};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

#[derive(Copy, Clone, Debug)]
struct Knot {
    row: i32,
    column: i32,
}

fn task1(content: &String) -> String {
    let tail_history = play_snake(content, 2);
    count_tail_history(tail_history).to_string()
}

fn task2(content: &String) -> String {
    let tail_history = play_snake(content, 10);
    count_tail_history(tail_history).to_string()
}

fn play_snake(content : &String, snake_length: usize) -> Vec<(i32, i32)> {
    let mut tail_history: Vec<(i32, i32)> = vec![];

    // Create a rope as array from head to tail
    let mut rope: Vec<Knot> = vec![Knot { row: 0, column: 0}; snake_length];

    for line in content.lines() {
        let (direction, steps_str) = line.split_once(" ").unwrap();
        let steps: usize = steps_str.parse().unwrap();

        for _step in 0..steps {
            match direction {
                "R" => {
                    debug!("Moving Right");
                    rope[0] = move_knot(rope[0], 0, 1);
                },
                "L" => {
                    debug!("Moving Left");
                    rope[0] = move_knot(rope[0], 0, -1);
                },
                "U" => {
                    debug!("Moving Up");
                    rope[0] = move_knot(rope[0], 1, 0);
                },
                "D" => {
                    debug!("Moving Down");
                    rope[0] = move_knot(rope[0], -1, 0);
                },
                _ => {
                    error!("This should never hapen!");
                },
            }
            
            rope = follow(rope);
            tail_history.push((rope[rope.len() - 1].row, rope[rope.len() - 1].column));
        } 
    }
    
    print_rope(&rope, &tail_history);
    tail_history
}


fn move_knot (knot: Knot, row: i32, column: i32) -> Knot {
    Knot {
        row: knot.row + row,
        column: knot.column + column,
    }
}

fn follow(mut rope: Vec<Knot>) -> Vec<Knot> {
    // Work on all knots except the first one as it is already moved
    for i in 1..rope.len() {
        let row_change = rope[i - 1].row - rope[i].row;
        let column_change = rope[i - 1].column - rope[i].column;

        if row_change.abs() + column_change.abs() > 2 {
            rope[i].row += row_change.signum();
            rope[i].column += column_change.signum();
        } else if row_change.abs() == 2 {
            rope[i].row += row_change.signum();
        } else if column_change.abs() == 2 {
            rope[i].column += column_change.signum();
        }
    }

    rope
}

fn print_rope (rope: &Vec<Knot>, tail_history: &Vec<(i32, i32)>) {
    // Configure printing grid
    let rows = 250;
    let cols = 250;

    for row in ((-rows/2)..(rows/2)).rev() {
        for column in (-cols/2)..(cols/2) {
            let mut printed = false;
            for i in 0..rope.len() {
                if rope[i].column == column && rope[i].row == row {
                    print!("{}", i);
                    printed = true;
                    break
                }
            }
            if !printed {
                if tail_history.contains(&(row, column)) {
                    print!("x");
                    printed = true;
                }
            }
            if !printed {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn count_tail_history(tail_history: Vec<(i32, i32)>) -> i32 {
    let mut result = 0;
    for _ in tail_history.iter().unique() {
        result += 1;
    }
    result
}


#[cfg(test)]
fn test_input() -> String {
    String::from(r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#)
}

#[test]
fn test_task1() {
   assert_eq!(task1(&test_input()), "13");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "1");
    assert_eq!(task2(&test_input2()), "36");
}
