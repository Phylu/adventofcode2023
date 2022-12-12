use grid::{Grid, grid};
use log::{debug, error};
use pathfinding::prelude::dijkstra;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node (usize, usize);

impl Node {
    fn neighbours(&self, map: &Grid<usize>) -> Vec<(Node, usize)> {    
        let x = &self.0;
        let y = &self.1;
        let mut neighbours: Vec<(Node, usize)> = vec![];

        // Top Node
        if *x > 0 {
          if map[*x-1][*y] <= map[*x][*y] + 1 {
            neighbours.push((Node(*x-1, *y), 1));
          }   
        }

        // Bottom Node
        if *x < map.rows() - 1 {
            if map[*x+1][*y] <= map[*x][*y] + 1 {
              neighbours.push((Node(*x+1, *y), 1));
            }   
          }

        // Lift  Node
        if *y > 0 {
            if map[*x][*y-1] <= map[*x][*y] + 1 {
              neighbours.push((Node(*x, *y-1), 1));
            }   
          }

        // Right Node
        if *y < map.cols() - 1 {
            if map[*x][*y+1] <= map[*x][*y] + 1 {
              neighbours.push((Node(*x, *y+1), 1));
            }   
          }

        neighbours
    }

}

fn task1(content: &String) -> String {
    
    let (map, start, finish) = parse_input(content);
    let result = dijkstra(&start, |n | n.neighbours(&map), |n| *n == finish).unwrap();
    String::from(result.1.to_string())
}

fn task2(content: &String) -> String {
    String::from("")
}

fn height_to_int(height: &char) -> usize {
    // Use radix value of 36 to remove 1-9 from the conversion
    height.to_digit(36).unwrap() as usize - 9
}

fn parse_input(content: &String) -> (Grid<usize>, Node, Node)  {

    let mut map: Grid<usize> = grid![[]];
    let mut row = 0;
    let mut start: Node = Node(0, 0);
    let mut finish: Node = Node(0, 0);

    for line in content.lines() {
        let mut col = 0;

        let mut heights: Vec<usize> = vec![];

        for char in line.chars() {
            if char == 'S' {
                heights.push(1);
                start = Node(row, col);
            } else if char == 'E' {
                heights.push(26);
                finish = Node(row, col);
            } else {
                heights.push(height_to_int(&char));
            }

        col += 1;
        }

        map.push_row(heights);
        row += 1;
    }

    (map, start, finish)

}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"

"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "31");
   //assert_eq!(task1(&test_input2()), "");
}

#[test]
fn test_task2() {
    //assert_eq!(task2(&test_input()), "");
    //assert_eq!(task2(&test_input2()), "");
}
