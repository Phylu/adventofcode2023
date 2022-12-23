use std::vec;

use log::{debug, error};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let mut positions = parse_input(content);
    let mut directions: Vec<Direction> = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        //draw(&positions);

        // Setup New Positions
        let potential_positions = get_potential_positions(&positions, &directions);
        positions = move_to_new_positions(&positions, &potential_positions);

        let changed_direction = directions.remove(0);
        directions.push(changed_direction);
    }

    draw(&positions);

    let (minx, miny, maxx, maxy) = square_edges(&positions);
    ((maxx + 1 - minx) * (maxy + 1 - miny) - positions.len() as i32).to_string()
}

fn task2(content: &String) -> String {
    
    let mut positions = vec![];
    let mut new_positions = parse_input(content);
    let mut directions: Vec<Direction> = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut rounds = -1;

    draw(&new_positions);

    while positions != new_positions {
        rounds += 1;
        
        positions = new_positions.clone();
        
        let potential_positions = get_potential_positions(&new_positions, &directions);
        new_positions = move_to_new_positions(&new_positions, &potential_positions);
        draw(&new_positions);
        
        let changed_direction = directions.remove(0);
        directions.push(changed_direction);
    }

    rounds.to_string()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn any_neighbours(self, positions: &Vec<Pos>) -> bool {
        positions.contains(&Pos::new(self.x - 1, self.y - 1))
            || positions.contains(&Pos::new(self.x - 1, self.y))
            || positions.contains(&Pos::new(self.x - 1, self.y + 1))
            || positions.contains(&Pos::new(self.x, self.y - 1))
            || positions.contains(&Pos::new(self.x, self.y + 1))
            || positions.contains(&Pos::new(self.x + 1, self.y - 1))
            || positions.contains(&Pos::new(self.x + 1, self.y))
            || positions.contains(&Pos::new(self.x + 1, self.y + 1))
    }

    fn walk(self, direction: &Direction, positions: &Vec<Pos>) -> Pos {
        let mut new_pos = self.clone();

        match direction {
            Direction::North => {
                if !positions.contains(&Pos::new(self.x - 1, self.y - 1))
                    && !positions.contains(&Pos::new(self.x - 1, self.y))
                    && !positions.contains(&Pos::new(self.x - 1, self.y + 1))
                {
                    new_pos = Pos::new(self.x - 1, self.y);
                }
            }
            Direction::West => {
                if !positions.contains(&Pos::new(self.x - 1, self.y - 1))
                    && !positions.contains(&Pos::new(self.x, self.y - 1))
                    && !positions.contains(&Pos::new(self.x + 1, self.y - 1))
                {
                    new_pos = Pos::new(self.x, self.y - 1);
                }
            }
            Direction::South => {
                if !positions.contains(&Pos::new(self.x + 1, self.y - 1))
                    && !positions.contains(&Pos::new(self.x + 1, self.y))
                    && !positions.contains(&Pos::new(self.x + 1, self.y + 1))
                {
                    new_pos = Pos::new(self.x + 1, self.y);
                }
            }
            Direction::East => {
                if !positions.contains(&Pos::new(self.x - 1, self.y + 1))
                    && !positions.contains(&Pos::new(self.x, self.y + 1))
                    && !positions.contains(&Pos::new(self.x + 1, self.y + 1))
                {
                    new_pos = Pos::new(self.x, self.y + 1);
                }
            }
        }

        new_pos
    }
}

fn draw(positions: &Vec<Pos>) {
    let (minx, miny, maxx, maxy) = square_edges(&positions);
    for x in 0..(maxx + 1 - minx) {
        for y in 0..(maxy + 1 - miny) {
            if positions.contains(&Pos::new(x + minx, y + miny)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    println!()
}

fn square_edges(positions: &Vec<Pos>) -> (i32, i32, i32, i32) {
    let mut minx = i32::MAX;
    let mut miny = i32::MAX;
    let mut maxx = i32::MIN;
    let mut maxy = i32::MIN;

    for p in positions {
        minx = std::cmp::min(minx, p.x);
        miny = std::cmp::min(miny, p.y);
        maxx = std::cmp::max(maxx, p.x);
        maxy = std::cmp::max(maxy, p.y);
    }

    println!("{}/{} - {}/{}", minx, miny, maxx, maxy);

    (minx, miny, maxx, maxy)
}

fn get_potential_positions(positions: &Vec<Pos>, directions: &Vec<Direction>) -> Vec<Pos> {
    let mut potential_positions: Vec<Pos> = vec![];

    for pos in positions {
        if !pos.any_neighbours(&positions) {
            // No Neighbors, So we are not moving.
            println!("Elf at pos {:?} has no neighbors and does not move", pos);
            potential_positions.push(*pos);
        } else {
            // We have a neighbour, so we try to move
            let mut new_pos = pos.clone();
            for direction in directions {
                new_pos = pos.walk(direction, &positions);
                if new_pos != *pos {
                    debug!(
                        "Elf at pos {:?} moves {:?} to {:?}",
                        pos, direction, new_pos
                    );
                    break;
                }
            }
            potential_positions.push(new_pos);
        }
    }

    potential_positions
}

fn move_to_new_positions(mut positions: &Vec<Pos>, potential_positions: &Vec<Pos>) -> Vec<Pos> {
    let mut new_positions: Vec<Pos> = vec![];
    for i in 0..positions.len() {
        let old_pos = positions[i];
        let new_pos = potential_positions[i];

        let mut position_checker = potential_positions.clone();
        position_checker.remove(i);

        if !position_checker.contains(&new_pos) {
            new_positions.push(new_pos);
        } else {
            new_positions.push(old_pos);
        }
    }

    new_positions
}

fn parse_input(content: &String) -> Vec<Pos> {
    let mut positions: Vec<Pos> = vec![];

    let mut x = 0;
    for line in content.lines() {
        let mut y = 0;
        for c in line.chars() {
            if c == '#' {
                positions.push(Pos::new(x, y));
            }
            y += 1
        }
        x += 1
    }

    positions
}

#[cfg(test)]
fn test_input() -> String {
    String::from(
        r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
"#,
    )
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(
        r#".....
..##.
..#..
.....
..##.
....."#,
    )
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input2()), "25");
    assert_eq!(task1(&test_input()), "110");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input2()), "3");
    assert_eq!(task2(&test_input()), "20");
}
