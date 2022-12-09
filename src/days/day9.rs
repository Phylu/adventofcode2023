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

    String::from("")

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
                        info!("Adding a new column to the left and moving head and tail position accordingly.");
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
                        info!("Adding a new row to the bottom and moving head and tail position accordingly.");
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

fn count_visited(grid: Grid<bool>) -> usize {

    let mut result = 0;

    for field in grid.iter() {
        if *field {
            result += 1;
        }
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

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "13");
}

#[test]
fn test_task2() {
    //assert_eq!(task2(&test_input()), "16");
}
