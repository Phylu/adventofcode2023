use std::{collections::HashMap, vec, cmp};

use itertools::min;
use log::{debug, error};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

#[derive(PartialEq, Clone, Copy, Debug, PartialOrd, Eq, Ord, Hash)]
struct Cube(i32, i32, i32);

fn task1(content: &String) -> String {
    let cubes = parse_input(content);
    free_sides(cubes).to_string()
}

fn task2(content: &String) -> String {
    let boundary = parse_input(content);
    let (min_pos, max_pos) = minmax(&boundary);
    let bucket = flood_fill(Cube(0, 0, 0), boundary, min_pos, max_pos);

    // Substracting all the sides of the container
    let len_x = 3 + max_pos.0 - min_pos.0;
    let len_y = 3 + max_pos.1 - min_pos.1;
    let len_z = 3 + max_pos.2 - min_pos.2;
    println!("Len_x: {}, Len_y: {}, Len_z: {}", len_x, len_y, len_z);
    let outside = 2 * (len_x * len_y + len_y * len_z + len_z * len_x);

    let fs = free_sides(bucket.clone());

    println!("Cubes: {}, Outside: {}, Free Sides: {}", bucket.len(), outside, fs);

    (fs - outside).to_string()
}

fn parse_input(content: &String) -> HashMap<Cube, bool> {
    let mut cubes: HashMap<Cube, bool> = HashMap::new();

    for line in content.lines() {
        let coords: Vec<&str> = line.split(",").collect();
        if coords.len() == 3 {
            cubes.insert(
                Cube(
                    coords[0].parse().unwrap(),
                    coords[1].parse().unwrap(),
                    coords[2].parse().unwrap(),
                ),
                true,
            );
        }
    }

    cubes
}

fn minmax(bucket: &HashMap<Cube, bool>) -> (Cube, Cube) {

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for (cube, _) in bucket {
        min_x = cmp::min(min_x, cube.0);
        min_y = cmp::min(min_y, cube.1);
        min_z = cmp::min(min_z, cube.2);
        max_x = cmp::max(max_x, cube.0);
        max_y = cmp::max(max_y, cube.1);
        max_z = cmp::max(max_z, cube.2);
    }

    println!("Min Pos: {}/{}/{}, Max Pos: {}/{}/{}", min_x, min_y, min_z, max_x, max_y, max_z);
    (Cube(min_x, min_y, min_z), Cube(max_x, max_y, max_z))
}

fn free_sides(cubes: HashMap<Cube, bool>) -> i32 {
    let mut free_sides = 0;

    for (c, _) in &cubes {
        for neighbor in [
            Cube(c.0 + 1, c.1, c.2),
            Cube(c.0 - 1, c.1, c.2),
            Cube(c.0, c.1 + 1, c.2),
            Cube(c.0, c.1 - 1, c.2),
            Cube(c.0, c.1, c.2 + 1),
            Cube(c.0, c.1, c.2 - 1),
        ] {
            if !cubes.contains_key(&neighbor) {
                free_sides += 1;
            }
        }
    }

    free_sides
}

fn flood_fill(cube: Cube, boundary: HashMap<Cube, bool>, min_pos: Cube, max_pos: Cube) -> HashMap<Cube, bool> {
    let mut bucket: HashMap<Cube, bool> = HashMap::new();
    let mut untested_cubes: Vec<Cube> = vec![];
    untested_cubes.push(cube);

    while untested_cubes.len() > 0 {
        let current = untested_cubes.pop().unwrap();

        if !bucket.contains_key(&current) && !boundary.contains_key(&current) {
            bucket.insert(current, true);

            for neighbor in [
                Cube(current.0 + 1, current.1, current.2),
                Cube(current.0 - 1, current.1, current.2),
                Cube(current.0, current.1 + 1, current.2),
                Cube(current.0, current.1 - 1, current.2),
                Cube(current.0, current.1, current.2 + 1),
                Cube(current.0, current.1, current.2 - 1),
            ] {
                if neighbor.0 >= min_pos.0 - 1 && neighbor.0 <= max_pos.0 + 1 && neighbor.1 >= min_pos.1 - 1 && neighbor.1 <= max_pos.1 + 1 && neighbor.2 >= min_pos.2 - 1 && neighbor.2 <= max_pos.2 + 1 {
                    untested_cubes.push(neighbor);
                }
            }
        }
    }

    println!("{:?}", bucket);
    println!("{}", bucket.len());

    //println!("{:?}", bucket);
    bucket
}

#[cfg(test)]
fn test_input() -> String {
    String::from(
        r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#,
    )
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"1,1,1
2,1,1
    "#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "64");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "58");
    //assert_eq!(task2(&test_input2()), "34");
}
