use std::{collections::HashSet, hash::Hash, str::FromStr};

use log::{debug, trace};
use parse_display::{Display, FromStr};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Display)]
#[display("({x}/{y})")]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn has_blizzard(self, blizzards: HashSet<Blizzard>) -> bool {
        let mut res = false;

        if blizzards.contains(&Blizzard {
            pos: self,
            direction: Direction::Right,
        }) || blizzards.contains(&Blizzard {
            pos: self,
            direction: Direction::Left,
        }) || blizzards.contains(&Blizzard {
            pos: self,
            direction: Direction::Up,
        }) || blizzards.contains(&Blizzard {
            pos: self,
            direction: Direction::Down,
        }) {
            res = true;
        }

        res
    }

    fn get_adjacent(self) -> Vec<Pos> {
        let mut adjacent: Vec<Pos> = vec![];

        adjacent.push(Pos {
            x: self.x - 1,
            y: self.y,
        });
        adjacent.push(Pos {
            x: self.x + 1,
            y: self.y,
        });
        adjacent.push(Pos {
            x: self.x,
            y: self.y - 1,
        });
        adjacent.push(Pos {
            x: self.x,
            y: self.y + 1,
        });

        adjacent
    }

    fn on_map(self, edges: Pos) -> bool {
        if self.x <= 0 || self.x >= edges.x || self.y <= 0 || self.y >= edges.y {
            false
        } else {
            true
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
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
    fn advance(self, edges: Pos) -> Blizzard {
        let mut res = self.clone();
        match self.direction {
            Direction::Right => {
                res.pos.y = {
                    if self.pos.y + 1 < edges.y {
                        self.pos.y + 1
                    } else {
                        1
                    }
                }
            }
            Direction::Left => {
                res.pos.y = {
                    if self.pos.y - 1 > 0 {
                        self.pos.y - 1
                    } else {
                        edges.y - 1
                    }
                }
            }
            Direction::Down => {
                res.pos.x = {
                    if self.pos.x + 1 < edges.x {
                        self.pos.x + 1
                    } else {
                        1
                    }
                }
            }
            Direction::Up => {
                res.pos.x = {
                    if self.pos.x - 1 > 0 {
                        self.pos.x - 1
                    } else {
                        edges.x - 1
                    }
                }
            }
        }
        res
    }
}

#[derive(Debug)]
struct State {
    pos: Pos,
    time: i32,
    blizzards: HashSet<Blizzard>,
}

fn task1(content: &String) -> String {
    let (blizzards, edges) = read_input(content);

    trace!("Blizzards: {:?}", blizzards);
    debug!("Edges: {}", edges);

    let start = Pos { x: 0, y: 1 };
    let end = Pos {
        x: edges.x,
        y: edges.y - 1,
    };

    debug!("Start: {}", start);
    debug!("End: {}", end);

    let result = traverse(edges, start, end, blizzards);

    result.0.to_string()
}

fn traverse(
    edges: Pos,
    start: Pos,
    end: Pos,
    blizzards: HashSet<Blizzard>,
) -> (i32, HashSet<Blizzard>) {
    let mut result = 0;

    // State:
    // Current Position, Timer, Blizzard State
    let mut queue: Vec<State> = vec![State {
        pos: start,
        time: 0,
        blizzards,
    }];
    let mut visited: HashSet<(Pos, i32)> = HashSet::new();
    let mut next_blizzards: HashSet<Blizzard> = HashSet::new();
    while result == 0 {
        // Get current state and add it to the states that have already been visited
        let current = queue.remove(0);

        debug!("Step: {}, Visiting Field {}", current.time, current.pos);
        //draw(edges, current.pos, &current.blizzards, start, end);

        // Advance the time
        let next_time = current.time + 1;

        // Advance all blizards
        next_blizzards = HashSet::new();
        for b in &current.blizzards {
            let newb = b.advance(edges);
            next_blizzards.insert(newb);
        }

        // Move to direction
        for next_pos in current.pos.get_adjacent() {
            // We are done!
            if next_pos == end {
                result = next_time;
                //draw(edges, next_pos, &next_blizzards, start, end);
                break;
            }

            if next_pos.on_map(edges)
                && !next_pos.has_blizzard(next_blizzards.clone())
                && !visited.contains(&(next_pos, next_time))
            {
                queue.push(State {
                    pos: next_pos,
                    time: next_time,
                    blizzards: next_blizzards.clone(),
                });

                visited.insert((next_pos, next_time));
            }
        }

        // Stay at the current position
        if !current.pos.has_blizzard(next_blizzards.clone())
            && !visited.contains(&(current.pos, next_time))
        {
            queue.push(State {
                pos: current.pos,
                time: next_time,
                blizzards: next_blizzards.clone(),
            });
            visited.insert((current.pos, next_time));
        }
    }

    (result, next_blizzards)
}

fn task2(content: &String) -> String {
    let (blizzards, edges) = read_input(content);
    let start = Pos { x: 0, y: 1 };
    let end = Pos {
        x: edges.x,
        y: edges.y - 1,
    };

    let (result1, blizzards1) = traverse(edges, start, end, blizzards);
    let (result2, blizzards2) = traverse(edges, end, start, blizzards1);
    let (result3, _) = traverse(edges, start, end, blizzards2);

    (result1 + result2 + result3).to_string()
}

fn read_input(content: &String) -> (HashSet<Blizzard>, Pos) {
    let mut blizzards: HashSet<Blizzard> = HashSet::new();

    let mut y = 0;
    let mut x = 0;
    for line in content.lines() {
        y = 0;
        for c in line.chars() {
            if c != '#' && c != '.' {
                let direction = Direction::from_str(&c.to_string()).unwrap();
                let pos = Pos { x, y };
                blizzards.insert(Blizzard { pos, direction });
            }
            y += 1;
        }
        x += 1;
    }

    (blizzards, Pos { x: x - 1, y: y - 1 })
}

fn draw(edges: Pos, pos: Pos, blizzards: &HashSet<Blizzard>, start: Pos, end: Pos) {
    // First Line:
    print!("#");

    if pos == start {
        print!("E");
    } else {
        print!(".");
    }

    for i in 0..(edges.y - 1) {
        print!("#");
    }
    println!();

    for x in 1..(edges.x) {
        // First Column:
        print!("#");

        for y in 1..(edges.y) {
            let mut blizzard_count = 0;
            let mut blizzard_character = "";

            if pos == (Pos { x, y }) {
                print!("E");
                continue;
            }

            if blizzards.contains(&Blizzard {
                pos: Pos { x, y },
                direction: Direction::Right,
            }) {
                blizzard_count += 1;
                blizzard_character = ">";
            }

            if blizzards.contains(&Blizzard {
                pos: Pos { x, y },
                direction: Direction::Left,
            }) {
                blizzard_count += 1;
                blizzard_character = "<";
            }

            if blizzards.contains(&Blizzard {
                pos: Pos { x, y },
                direction: Direction::Up,
            }) {
                blizzard_count += 1;
                blizzard_character = "^";
            }

            if blizzards.contains(&Blizzard {
                pos: Pos { x, y },
                direction: Direction::Down,
            }) {
                blizzard_count += 1;
                blizzard_character = "v";
            }

            if blizzard_count > 1 {
                print!("{}", blizzard_count);
            } else if blizzard_count == 1 {
                print!("{}", blizzard_character);
            } else {
                print!(".");
            }
        }

        // Last Column:
        println!("#");
    }

    // Last Line
    for _ in 0..(edges.y - 1) {
        print!("#");
    }

    if pos == end {
        print!("E");
    } else {
        print!(".");
    }

    println!("#\n");
}

#[cfg(test)]
fn test_input() -> String {
    String::from(
        r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"#,
    )
}

#[test]
fn test_blizzards() {
    let edges = Pos { x: 4, y: 4 };

    let mut b = Blizzard {
        pos: Pos { x: 3, y: 3 },
        direction: Direction::Right,
    };
    assert_eq!(
        b.advance(edges),
        Blizzard {
            pos: Pos { x: 3, y: 1 },
            direction: Direction::Right
        }
    );

    b.direction = Direction::Left;
    assert_eq!(
        b.advance(edges),
        Blizzard {
            pos: Pos { x: 3, y: 2 },
            direction: Direction::Left
        }
    );
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "18");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "54");
}
