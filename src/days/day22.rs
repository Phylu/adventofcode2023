use grid::{Grid, grid};
use parse_display::{Display, FromStr};
use log::{debug, error};
use rotate_enum::RotateEnum;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let (map, directions) = parse_input(content);
    let mut pos = Pos {
        row: 1,
        column: 1,
        facing: Facing::Right,
    };

    for d in directions {
        pos = pos.step(&map, d);
    }

    pos.get_result().to_string()
}

fn task2(content: &String) -> String {
    String::from("")
}

#[derive(Debug, FromStr, Display)]
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

#[derive(Debug, RotateEnum)]
enum Facing {
    Top,
    Right,
    Bottom,
    Left,
}

impl Facing {
    fn to_i32(self) -> i32 {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Top => 3
        }
    }
}

#[derive(Debug)]
struct Pos {
    row: i32,
    column: i32,
    facing: Facing, 
}

impl Pos {
    fn step(mut self, map: &Grid<Tile>, direction: Direction) -> Pos {

        match direction {
            Direction::Steps(s) => {
                for i in 0..s {
                    let mut column = self.column;
                    let mut row = self.row;
                    match self.facing {
                        Facing::Right => {
                            
                        
                        },
                        Facing::Bottom => {

                        },
                        Facing::Left => {
                            
                        },
                        Facing::Top => {

                        },
                    }
                }
                println!("Moving {} steps forward. New position is: {:?}", s, self);
            }
            Direction::Left => {
                self.facing = self.facing.prev();
                println!("Turning Left. New position is: {:?}", self);
            }
            Direction::Right => {
                self.facing = self.facing.next();
                println!("Turning Right. New position is: {:?}", self);
            }
        }
        println!();

        self
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
            steps.push(c);
        }
    }

    debug!("{:?}", map);
    debug!("{:?}", directions);

    (map, directions)
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
    assert_eq!(task2(&test_input2()), "");
}
