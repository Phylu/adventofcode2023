use std::fmt;

use grid::{Grid, grid};
use parse_display::{Display, FromStr};
use log::{debug, trace};
use rotate_enum::RotateEnum;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let (map, directions) = parse_input(content);

    let mut c = 0;
    for i in 0..map.cols() {
        if map[0][i] == Tile::Open {
            break;
        }
        c += 1;
    }

    let mut pos = Pos {
        row: 0,
        column: c,
        facing: Facing::Right,
    };

    for d in directions {
        pos = pos.step(&map, d);
    }

    draw(&map, pos);

    pos.get_result().to_string()
}

fn task2(content: &String) -> String {
    String::from("")
}

#[derive(Debug, FromStr, Display, PartialEq, Eq)]
enum Tile {
    #[display(" ")]
    None,
    #[display(".")]
    Open,
    #[display("#")]
    Wall,
}


#[derive(Debug, FromStr, Display)]
enum Direction {
    #[display("{0}")]
    Steps(i32),
    #[display("L")]
    Left,
    #[display("R")]
    Right,
}

#[derive(Debug, RotateEnum, Clone, Copy)]
enum Facing {
    Top,
    Right,
    Bottom,
    Left,
}

impl fmt::Display for Facing {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match self {
            Facing::Top => "^",
            Facing::Right => ">",
            Facing::Bottom => "v",
            Facing::Left => "<",
        };
        write!(f, "{}", d)
    }
}

impl Facing {
    fn to_i32(self) -> i32 {
        match self {
            Facing::Right => 0,
            Facing::Bottom => 1,
            Facing::Left => 2,
            Facing::Top => 3
        }
    }

    fn get_direction(self) -> (i32, i32) {
        let mut add_rows = 0;
        let mut add_columns = 0;
        match self {
            Facing::Right => {
                add_columns = 1;
            },
            Facing::Bottom => {
                add_rows = 1;
            },
            Facing::Left => {
                add_columns = -1;
            },
            Facing::Top => {
                add_rows = -1;
            },
        }

        (add_rows, add_columns)
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: i32,
    column: i32,
    facing: Facing, 
}

impl Pos {
    fn add(mut self, map: &Grid<Tile>, rows: i32, columns: i32) -> Pos {

        self.row = (self.row + rows).rem_euclid(map.rows() as i32);
        self.column = (self.column + columns).rem_euclid(map.cols() as i32);

        self
    }

    fn step(self, map: &Grid<Tile>, direction: Direction) -> Pos {
        let mut pos = self.clone();

        match direction {
            Direction::Steps(s) => {
                let (add_rows, add_columns) = &self.facing.get_direction();
                for i in 0..s {  
                    let mut new_pos = pos.add(map, *add_rows, *add_columns);
                    while map[new_pos.row as usize][new_pos.column as usize] == Tile::None {
                        new_pos = new_pos.add(map, *add_rows, *add_columns);
                        trace!("New Position: {:?}", new_pos);
                    }
                    if map[new_pos.row as usize][new_pos.column as usize] == Tile::Open {
                        pos = new_pos
                    }
                    if map[new_pos.row as usize][new_pos.column as usize] == Tile::Wall {
                        break;
                    }
                }
                debug!("Moving {} steps forward. New position is: {:?}", s, pos);
            }
            Direction::Left => {
                pos.facing = self.facing.prev();
                debug!("Turning Left. New position is: {:?}", pos);
            }
            Direction::Right => {
                pos.facing = self.facing.next();
                debug!("Turning Right. New position is: {:?}", pos);
            }
        }

        pos
    }

    fn get_result(self) -> i32 {
        ((self.row + 1) * 1000) + ((self.column + 1) * 4) + self.facing.to_i32()
    }
}

fn parse_input(content: &String) -> (Grid<Tile>, Vec<Direction>) {

    let (map_input, directions_input) = content.split_once("\n\n").unwrap();

    // Calculate Grid Size
    let mut columns = 0;
    for line in map_input.lines() {
        columns = std::cmp::max(columns, line.len());
    }

    // Setup Grid
    let mut map: Grid<Tile> = grid![[]];
    for line in map_input.lines() {
        let mut tiles: Vec<Tile> = vec![];
        for c in line.chars() {
            let tile: Tile = c.to_string().parse().unwrap();
            tiles.push(tile);
        }
        for i in tiles.len()..columns {
            tiles.push(Tile::None);
        }

        map.push_row(tiles);
    }

    // Setup Directions
    let mut directions: Vec<Direction> = vec![];
    let mut steps = "".to_string();
    for c in directions_input.chars() {
        if c == 'R' || c == 'L' {
            if steps.len() > 0 {
                let direction: Direction = steps.parse().unwrap();
                directions.push(direction);
            }
            let direction: Direction = c.to_string().parse().unwrap();
            directions.push(direction);
            steps = "".to_string();
        } else {
            if c != '\n' {
                steps.push(c);
            }
        }
    }
    if steps.len() > 0 {
        let direction: Direction = steps.parse().unwrap();
        directions.push(direction);
    }

    debug!("{:?}", map);
    debug!("{:?}", directions);

    (map, directions)
}

fn draw(map: &Grid<Tile>, pos: Pos) {
    for row in 0..map.rows() {
        for column in 0..map.cols() {
            if (pos.row as usize) == row && (pos.column as usize) == column {
                print!("{}", pos.facing);
            } else {
                print!("{}", map[row][column]);
            }
        }
        println!()
    }
    println!()
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"

"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "6032");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "");
}
