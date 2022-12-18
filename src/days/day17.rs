use grid::{Grid, grid};
use log::{debug, error};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

const MAX_ROCKS: usize = 2022;
const ROCK_ORDER: [Shape; 5] = [Shape::Row, Shape::Plus, Shape::L, Shape::Column, Shape::Block];
const CAVE_WIDTH: usize = 7;

fn task1(content: &String) -> String {

    let jet_pattern = read_input(content);
    let jet_counter: usize = 0;
    let mut cave: Grid<bool> = grid![];

    for i in 0..MAX_ROCKS {
        let shape = &ROCK_ORDER[i % 5];
        let mut rock = Rock::new(shape, height(&cave));

        // Move Rock to the correct position
        loop {
            let direction = &jet_pattern[jet_counter % jet_pattern.len()];
            rock = rock.side(direction);
            
            // Check if down is even possible and exit if not.
            //rock.down();
            break
        }
        
        // Add final rock position to our grid        
        cave = place_rock(cave, rock);

        draw(&cave);
        break

    }
     
    String::from("")
}

fn task2(content: &String) -> String {
    String::from("")
}

#[derive(PartialEq, Debug)]
enum Shape {
    Row,
    Plus,
    L,
    Column,
    Block,
}

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}

// Pos(row, column)
#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug, Clone)]
struct Rock {
    positions: Vec<Pos>
}

impl Rock {
    fn new(shape: &Shape, height: usize) -> Rock {
        let mut rock = Rock {
            positions: vec![],
        };

        match shape {
            Shape::Row => {
                let x = height + 3;
                rock.positions = vec![Pos(x, 2), Pos(x, 3), Pos(x, 4), Pos(x, 5)]
            }
            _ => {}
        }

        rock
    }

    fn side(self, direction: &Direction) -> Rock {
        let mut new_rock = self.clone();
        for i in 0..self.positions.len() {

            if (*direction == Direction::Left && self.positions[i].1 == 0) || 
                (*direction == Direction::Right && self.positions[i].1 == CAVE_WIDTH) {
                    // We are already at the edge. Don't move.
                    return self;
                }

            if *direction == Direction::Left {
                new_rock.positions[i].1 -= 1;
            } else if *direction == Direction::Right {
                new_rock.positions[i].1 += 1;
            }
        }

        new_rock
    }
    
    fn down(){}
}

fn read_input(content: &String) -> Vec<Direction> {
    let mut jet_pattern: Vec<Direction> = vec![];
    
    for c in content.chars() {
        match c {
            '<' => jet_pattern.push(Direction::Left),
            '>' => jet_pattern.push(Direction::Right),
            _ => {}
        }
    }

    jet_pattern
}

fn place_rock(mut cave: Grid<bool>, rock: Rock) -> Grid<bool> {

    for p in rock.positions {
        let additional_rows = p.0 + 1 - cave.rows();
        if additional_rows > 0 {
            for _ in 0..additional_rows {
                cave.push_row(vec![false; CAVE_WIDTH]);
            }
        }

        cave[p.0][p.1] = true;
    }

    cave
}

fn height(cave: &Grid<bool>) -> usize {
    for row in (0..cave.rows()).rev() {
        for col in 0..CAVE_WIDTH {
            if cave[row][col] {
                return row;
            }
        }
    }

    0
}

fn draw(cave: &Grid<bool>) {
    for i in (0..cave.rows()).rev() {
        print!("{:04} ", i);
        for j in cave[i].iter() {
            match j {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"

"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "3068");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "");
    assert_eq!(task2(&test_input2()), "");
}
