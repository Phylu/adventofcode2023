use std::vec;

use log::{debug, error};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

#[derive(PartialEq, Clone, Copy, Debug, PartialOrd, Eq, Ord)]
struct Cube(i32, i32, i32);

fn task1(content: &String) -> String {
    let cubes = parse_input(content);
    free_sides(cubes).to_string()
}

fn task2(content: &String) -> String {
    let boundary = parse_input(content);
    let bucket = flood_fill(Cube(0, 0, 0), boundary);

    // Substracting all the sides of the container going from 0 - 22 on the indices
    (free_sides(bucket) - (23 * 23 * 6)).to_string()
}

fn parse_input(content: &String) -> Vec<Cube> {
    let mut cubes: Vec<Cube> = vec![];
    for line in content.lines() {
        let coords: Vec<&str> = line.split(",").collect();
        if coords.len() == 3 {
            cubes.push(Cube(
                coords[0].parse().unwrap(),
                coords[1].parse().unwrap(),
                coords[2].parse().unwrap(),
            ));
        }
    }

    cubes
}

fn free_sides(cubes: Vec<Cube>) -> i32 {
    let mut free_sides = 0;

    'outer: for i in 0..cubes.len() {
        let mut cube_free_sides = 6;
        for j in 0..cubes.len() {
            if (cubes[i].0 - cubes[j].0).abs()
                + (cubes[i].1 - cubes[j].1).abs()
                + (cubes[i].2 - cubes[j].2).abs()
                == 1
            {
                cube_free_sides -= 1;
            }

            if cube_free_sides == 0 {
                continue 'outer;
            }
        }

        free_sides += cube_free_sides;
    }

    free_sides
}

fn flood_fill(cube: Cube, boundary: Vec<Cube>) -> Vec<Cube> {
    let mut bucket: Vec<Cube> = vec![];
    let mut untested_cubes: Vec<Cube> = vec![];
    untested_cubes.push(cube);

    while untested_cubes.len() > 0 {
        let current = untested_cubes.pop().unwrap();

        if !bucket.contains(&current) && !boundary.contains(&current) {
            bucket.push(current);

            if current.0 > 0 {
                let new_cube = Cube(current.0 - 1, current.1, current.2);
                untested_cubes.push(new_cube);
            }
            if current.0 < 22 {
                let new_cube = Cube(current.0 + 1, current.1, current.2);
                untested_cubes.push(new_cube);
            }

            if current.1 > 0 {
                let new_cube = Cube(current.0, current.1 - 1, current.2);
                untested_cubes.push(new_cube);
            }
            if current.1 < 22 {
                let new_cube = Cube(current.0, current.1 + 1, current.2);
                untested_cubes.push(new_cube);
            }

            if current.2 > 0 {
                let new_cube = Cube(current.0, current.1, current.2 - 1);
                untested_cubes.push(new_cube);
            }
            if current.2 < 22 {
                let new_cube = Cube(current.0, current.1, current.2 + 1);
                untested_cubes.push(new_cube);
            }
        }
    }

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

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "64");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "58");
}
