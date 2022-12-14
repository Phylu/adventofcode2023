use std::{cmp, vec};
use grid::{Grid, grid};
use log::{debug, error};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {

    let (mut cave, min_x, max_x, max_y) = parse_input(content);
    draw(&cave, min_x, max_x, max_y);

    let mut i = 0;
    loop {
        let potential_cave = spawn_sand(cave.clone(), min_x, max_x, max_y);
        match potential_cave {
            Some(c) => cave = c,
            None => break,
        }
        // draw(&cave, min_x, max_x, max_y);
        i += 1;

        // TODO: Remove. For debugging only.
        //if i == 50 {
        //    break
        //}
    }

    draw(&cave, min_x, max_x, max_y);

    i.to_string()
}

fn task2(content: &String) -> String {
    
    let (mut cave, mut min_x, mut max_x, mut max_y) = parse_input(content);
    
    // Increase Grid Size
    // We are setting this to a nice value to be still able to draw
    let grow: usize = 150;
    for _ in 0..grow {
        cave.push_col(vec![Material::Air; cave.rows()]);
        cave.insert_col(0, vec![Material::Air; cave.rows()]);
    }
    min_x -= grow;
    max_x += grow;

    cave.push_row(vec![Material::Air; cave.cols()]);
    cave.push_row(vec![Material::Rock; cave.cols()]);
    max_y += 2;

    draw(&cave, min_x, max_x, max_y);

    let mut i = 0;
    loop {
        let potential_cave = spawn_sand(cave.clone(), min_x, max_x, max_y);
        match potential_cave {
            Some(c) => cave = c,
            None => break,
        }
        //draw(&cave, min_x, max_x, max_y);
        i += 1;

        if cave[HOLE.1][HOLE.0 - min_x] == Material::Sand {
            break
        }
    }

    draw(&cave, min_x, max_x, max_y);

    i.to_string()

}


const HOLE: (usize, usize) = (500, 0);

#[derive(PartialEq, Debug, Clone, Copy)]
enum Material {
    Air,
    Sand,
    Rock,
    Hole
}

fn parse_input(content: &String) -> (Grid<Material>, usize, usize, usize) {
    let mut rocks: Vec<(usize, usize)> = vec![];
    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    for line in content.lines() {
        let mut points = line.split(" -> ").into_iter().peekable();
        loop {
            let point = points.next();
            match point {
                Some(p) => {
                    let next = points.peek();
                    match next {
                        Some(n) => {
                            let (x, y) = p.split_once(",").unwrap();
                            let (xn, yn) = n.split_once(",").unwrap();
                            let mut this_rocks = get_rocks(x.parse().unwrap(), y.parse().unwrap(), xn.parse().unwrap(), yn.parse().unwrap());
                            rocks.append(&mut this_rocks);

                            min_x = cmp::min(cmp::min(min_x, x.parse().unwrap()), xn.parse().unwrap());
                            max_x = cmp::max(cmp::max(max_x, x.parse().unwrap()), xn.parse().unwrap());
                            min_y = cmp::min(cmp::min(min_y, y.parse().unwrap()), yn.parse().unwrap());
                            max_y = cmp::max(cmp::max(max_y, y.parse().unwrap()), yn.parse().unwrap());
                        },
                        None => {
                            break;
                        }
                    }
                },
                None => { 
                    break;
                },
            }
        }
    }

    // rocks are filled now
    debug!("Min: {}/{}, Max: {}/{}", min_x, min_y, max_x, max_y);
    debug!("Rocks: {:?}", rocks);

    let columns = max_x - min_x + 1;

    let mut cave: Grid<Material> = grid![[]];
    for i in 0..max_y + 1 {
        let mut row: Vec<Material> = vec![];
        for j in 0..columns {
            if (j+min_x, i) == HOLE {
                row.push(Material::Hole);
            } else if rocks.contains(&(j + min_x, i)) {
                row.push(Material::Rock);
            } else {
                row.push(Material::Air);
            }
        }
        cave.push_row(row);
    }

    return (cave, min_x, max_x, max_y);
}

fn spawn_sand(mut cave: Grid<Material>, min_x: usize, max_x: usize, max_y: usize) -> Option<Grid<Material>> {

    // Spawn new Sand
    let mut sand_pos = HOLE;
    let mut sand_pos_prev = HOLE;
    let mut first_drop = true;

    // Move the Sand
    while first_drop || sand_pos != sand_pos_prev {
        first_drop = false;

        sand_pos_prev = sand_pos;
        let potenial_sand_pos = move_sand(&cave, sand_pos, min_x, max_x, max_y);
        match potenial_sand_pos {
            Some(p) => sand_pos = p,
            None => return None
        }
    }
    cave[sand_pos.1][sand_pos.0 - min_x] = Material::Sand;

    Some(cave)
}

fn move_sand(cave: &Grid<Material>, sand_pos: (usize, usize), min_x: usize, max_x: usize, max_y: usize) -> Option<(usize, usize)> {

    let grid_x = sand_pos.0 - min_x;
    let grid_y = sand_pos.1;

    // Check for End of Movement
    // We are in the last row, so we fall off the grid
    if grid_y == max_y {
        debug!("Last line: {}", grid_y);
        return None;
    }
    // Check if Move Down possible
    if is_air(&cave, grid_x, grid_y + 1) {
        // Move Down
        debug!("Move Down to: ({}/{})", sand_pos.0, sand_pos.1 + 1);
        return Some((sand_pos.0, sand_pos.1 + 1));
    }

    // Check for End of Movement
    // We are in the leftmost column, so we fall off the grid
    if grid_x == 0 {
        debug!("Leftmost Line: {}", grid_x);
        return None;
    }
    // Check if Left/Down possible
    if is_air(&cave, grid_x - 1, grid_y + 1) {
        // Move Left/Down
        debug!("Move Left/Down to: ({}/{})", sand_pos.0 - 1, sand_pos.1 + 1);
        return Some((sand_pos.0 - 1, sand_pos.1 + 1));
    }

    // Check for End of Movement
    // We are in the rightmost column, so we fall off the grid
    if grid_x == cave.cols() {
        debug!("Rightmost Line: {}", grid_x);
        return None;
    }
    // Check if Right/Down possible
    if is_air(&cave, grid_x + 1, grid_y + 1) {
        // Move Right/Down
        debug!("Move Right/Down to: ({}/{})", sand_pos.0 + 1, sand_pos.1 + 1);
        return Some((sand_pos.0 + 1, sand_pos.1 + 1));
    }

    Some(sand_pos)
}

fn is_air(cave: &Grid<Material>, grid_x: usize, grid_y: usize) -> bool {
    cave[grid_y][grid_x] == Material::Air
}

fn draw(cave: &Grid<Material>, min_x: usize, max_x: usize, max_y: usize) {
    for cc in 0..3 {
        print!("  ");
        for c in 0..cave.cols() {
            let column = (min_x + c).to_string();
            print!("{}", column.chars().nth(cc).unwrap());
        }
        println!();
    }

    for i in 0..cave.rows() {
        print!("{} ", i);
        for j in cave[i].iter() {
            match j {
                Material::Air => print!("."),
                Material::Hole => print!("+"),
                Material::Rock => print!("#"),
                Material::Sand => print!("o"),
            }
        }
        println!();
    }

    println!();
}

fn get_rocks(x: usize, y: usize, xn: usize, yn: usize) -> Vec<(usize, usize)> {
    let mut rocks: Vec<(usize, usize)> = vec![];

    if x < xn {
        for i in x..xn + 1 {
            rocks.push((i, y));
        }
    } else if x > xn {
        for i in xn..x + 1 {
            rocks.push((i, y));
        }
    } else if y <= yn {
        for i in y..yn + 1 {
            rocks.push((x, i));
        }
    } else if y >= yn {
        for i in yn..y + 1 {
            rocks.push((x, i))
        }
    }

    debug!("Rocks between {}/{} & {}/{} are: {:?}", x, y, xn, yn, rocks);

    rocks
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"

"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "24");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "93");
}
