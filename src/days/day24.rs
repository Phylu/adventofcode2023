use std::{str::FromStr, collections::HashSet, hash::Hash};

use log::{debug, error};
use parse_display::{Display, FromStr};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Blizzard {
    pos: Pos,
    direction: Direction,
}

#[derive(Debug, FromStr, Display, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    #[display(">")]
    Right,
    #[display("v")]
    Down,
    #[display("<")]
    Left,
    #[display("^")]
    Up,
}

impl Blizzard {
    fn minute(mut self, edges: Pos) -> Blizzard {
        match self.direction {
            Direction::Right => self.pos.y = self.pos.y + 1 % edges.y,
            Direction::Left => self.pos.y = self.pos.y - 1 % edges.y,
            Direction::Down => self.pos.y = self.pos.y + 1 % edges.y,
            Direction::Up => self.pos.y = self.pos.y + 1 % edges.y,
        }
        self
    }
}

fn task1(content: &String) -> String {
    let (blizzards, edges) = read_input(content);

    println!("{:?}", blizzards);
    println!("{:?}", edges);

    String::from("")
}

fn task2(content: &String) -> String {
    String::from("")
}

fn read_input(content: &String) -> (HashSet<Blizzard>, Pos) {
    let mut blizzards : HashSet<Blizzard> = HashSet::new();

    let mut y = 0;
    let mut x = 0;
    for line in content.lines() {
        y = 0;
        for c in line.chars() {
            if c != '#' && c != '.' {
                let direction = Direction::from_str(&c.to_string()).unwrap();
                let pos = Pos{x, y};
                blizzards.insert(Blizzard{pos, direction});
            }
            y += 1;
        }
        x += 1;
    }

    (blizzards, Pos{x: x - 1, y: y - 1})
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#
"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"

"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "18");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "");
    assert_eq!(task2(&test_input2()), "");
}
