use itertools::Itertools;
use log::{debug, error, info, trace};
use grid::*;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {

    let grid = construt_grid(content);
    count_visited(grid).to_string()

}

fn task2(content: &String) -> String {

    let tail_history = play_snake(content);
    count_tail_history(tail_history).to_string()

}

fn construt_grid(content : &String) -> Grid<bool> {
    let mut head_row = 0;
    let mut head_column = 0;
    let mut tail_row = 0;
    let mut tail_column = 0;

    // Initialize the grid where the tail visited the starting position
    let mut grid = grid![[true]];

    for line in content.lines() {
        let (direction, steps_str) = line.split_once(" ").unwrap();
        let steps: usize = steps_str.parse().unwrap();

        for _step in 0..steps {
            match direction {
                "R" => {
                    debug!("Moving Right");
                    head_column += 1;
                    if head_column >= grid.cols() {
                        grid.push_col(vec![false; grid.rows()]);
                    }
                    (tail_row, tail_column) = follow_right(head_row, head_column, tail_row, tail_column);
                },
                "L" => {
                    debug!("Moving Left");
                    if head_column == 0 {
                        debug!("Adding a new column to the left and moving head and tail position accordingly.");
                        grid.insert_col(0, vec![false; grid.rows()]);
                        head_column += 1;
                        tail_column += 1;
                    }
                    head_column -= 1;
                    (tail_row, tail_column) = follow_left(head_row, head_column, tail_row, tail_column);
                },
                "U" => {
                    debug!("Moving Up");
                    head_row += 1;
                    if head_row >= grid.rows() {
                        grid.push_row(vec![false; grid.cols()]);
                    }
                    (tail_row, tail_column) = follow_up(head_row, head_column, tail_row, tail_column);
                },
                "D" => {
                    debug!("Moving Down");
                    if head_row == 0 {
                        debug!("Adding a new row to the bottom and moving head and tail position accordingly.");
                        grid.insert_row(0, vec![false; grid.cols()]);
                        head_row += 1;
                        tail_row += 1;
                    }
                    head_row -= 1;
                    (tail_row, tail_column) = follow_down(head_row, head_column, tail_row, tail_column);
                },
                _ => {
                    error!("This should never hapen!");
                },
            }
            grid[tail_row][tail_column] = true;
            trace!("{:?}", grid);
        } 
    }

    grid
}

fn follow_up(head_row: usize, head_column: usize, mut tail_row: usize, mut tail_column: usize)  -> (usize, usize) {
    if head_row > tail_row + 1 {
        tail_row = head_row - 1;
        tail_column = head_column;
    }
    (tail_row, tail_column)
}

fn follow_down(head_row: usize, head_column: usize, mut tail_row: usize, mut tail_column: usize)  -> (usize, usize) {
    if head_row + 1 < tail_row {
        tail_row = head_row + 1;
        tail_column = head_column;
    }
    (tail_row, tail_column)
}

fn follow_right(head_row: usize, head_column: usize, mut tail_row: usize, mut tail_column: usize)  -> (usize, usize) {
    if head_column > tail_column + 1 {
        tail_column = head_column - 1;
        tail_row = head_row;
    }
    (tail_row, tail_column)
}

fn follow_left(head_row: usize, head_column: usize, mut tail_row: usize, mut tail_column: usize)  -> (usize, usize) {
    if head_column + 1 < tail_column {
        tail_column = head_column + 1;
        tail_row = head_row;
    }
    (tail_row, tail_column)
}

#[derive(Copy, Clone, Debug)]
struct Knot {
    row: i32,
    column: i32,
}

fn play_snake(content : &String) -> Vec<(i32, i32)> {
    let mut tail_history: Vec<(i32, i32)> = vec![];

    // Create a rope as array from head to tail
    let mut rope: [Knot; 10] = [Knot { row: 0, column: 0}; 10];

    // Initialize the grid where the tail visited the starting position
    let mut grid = grid![[true]];

    for line in content.lines() {
        println!("{}", line);
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
            tail_history.push((rope[9].row, rope[9].column));
            // print_rope(grid.rows(), grid.cols(), rope);
        } 
    }

    tail_history
}


fn move_knot (knot: Knot, row: i32, column: i32) -> Knot {
    Knot {
        row: knot.row + row,
        column: knot.column + column,
    }
}

fn follow(mut rope: [Knot; 10]) -> [Knot; 10] {
    // Work on all knots except the first one as it is already moved
    for i in 1..10 {
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

fn print_rope (rows: usize, cols: usize, rope: [Knot; 10]) {
    for row in (0..10).rev() {
        print!("{} ", row);
        for column in 0..10 {
            let mut printed = false;
            for i in 0..rope.len() {
                if rope[i].column == column && rope[i].row == row {
                    print!("{}", i);
                    printed = true;
                    break
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

fn count_visited(grid: Grid<bool>) -> usize {

    let mut result = 0;

    for field in grid.iter() {
        if *field {
            result += 1;
        }
    }

    result
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
   // assert_eq!(task1(&test_input()), "13");
}

#[test]
fn test_task2() {
    //assert_eq!(task2(&test_input()), "2");
    assert_eq!(task2(&test_input2()), "36");
}
