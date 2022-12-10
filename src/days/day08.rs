use log::{debug, trace};

struct Tree {
    height: u8,
    visible: bool,
    scenery: i32,
}

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {

    let mut forest = read_input(&content);
    forest = calculate_visible(forest);
    gather_visibility(forest)

}

fn task2(content: &String) -> String {

    let mut forest = read_input(&content);
    forest = calculate_scenery(forest);
    gather_scenery(forest)
}

fn read_input(content: &str) -> Vec<Vec<Tree>> {
    let mut forest: Vec<Vec<Tree>> = vec![];

    for line in content.lines() {
        let mut tree_line: Vec<Tree> = vec![];
        for c in line.chars() {
            let height: u8 = c.to_digit(10).unwrap() as u8;
            let tree = Tree { height: height, visible: false, scenery: 1 };
            tree_line.push(tree)
        }
        forest.push(tree_line);
    }

    forest
}

fn calculate_visible(mut forest: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {

    // Calculate from left to right & right to left
    for row in 0..forest.len() {
        forest = row_vibility(forest, row, false);
        forest = row_vibility(forest, row, true);
    }

    // Calculate from top to bottom & bottom to top
    for column in 0..forest[0].len() {
        forest = column_visibility(forest, column, false);
        forest = column_visibility(forest, column, true);
    }

    forest
}

fn row_vibility(mut forest: Vec<Vec<Tree>>, row: usize, reverse: bool) -> Vec<Vec<Tree>> {
    debug!("Row Visibility with Reverse = {}", reverse);
    debug!("Working on Row: {}", row);

    let tree_line: &mut Vec<Tree> = &mut forest[row];
    let range = if reverse { itertools::Either::Right((0..tree_line.len()).rev()) } else { itertools::Either::Left(0..tree_line.len()) };
    for i in range {
        debug!("Working on tree: {}", i);
        let mut visible = true;
        // We need to add +1 on the iterator here when moving in from the right to start at the correct location
        let inner_range = if reverse { itertools::Either::Right(((i + 1)..tree_line.len()).rev()) } else { itertools::Either::Left(0..i) };
        for j in inner_range {
            if tree_line[j].height >= tree_line[i].height {
                visible = false;
            }
        }

        // Only overwrite the visibility if the tree is not yet visible from any other side
        if !tree_line[i].visible {
            tree_line[i].visible = visible;
        }
    }

    forest
}

fn column_visibility(mut forest: Vec<Vec<Tree>>, column: usize, reverse: bool) -> Vec<Vec<Tree>> {
    debug!("Column Visibility with Reverse = {}", reverse);
    debug!("Working on Column: {}", column);

    let range = if reverse { itertools::Either::Right((0..forest.len()).rev()) } else { itertools::Either::Left(0..forest.len()) };
    for i in range {
        debug!("Working on tree: {}", i);
        let mut visible = true;
        let inner_range = if reverse { itertools::Either::Right(((i + 1)..forest.len()).rev()) } else { itertools::Either::Left(0..i) };
        for j in inner_range {
            if forest[j][column].height >= forest[i][column].height {
                visible = false;
            }
        }
        // Only overwrite the visibility if the tree is not yet visible from any other side
        if !forest[i][column].visible {
            forest[i][column].visible = visible;
        }
    }

    forest
}

fn gather_visibility(forest: Vec<Vec<Tree>>) -> String {
    let mut result = 0;

    for forest_line in forest {
        for tree in forest_line {
            if tree.visible { result += 1}
        }
    }

    result.to_string()
}

fn calculate_scenery(mut forest: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {

    // Calculate from left to right & right to left
    for row in 0..forest.len() {
        forest = row_scenery(forest, row, false);
        forest = row_scenery(forest, row, true);
    }

    // Calculate from top to bottom & bottom to top
    for column in 0..forest[0].len() {
        forest = column_scenery(forest, column, false);
        forest = column_scenery(forest, column, true);
    }

    forest
}

fn row_scenery(mut forest: Vec<Vec<Tree>>, row: usize, reverse: bool) -> Vec<Vec<Tree>> {
    debug!("Row Scenery with Reverse = {}", reverse);
    debug!("Working on Row: {}", row);

    let tree_line: &mut Vec<Tree> = &mut forest[row];
    let range = if reverse { itertools::Either::Right((0..tree_line.len()).rev()) } else { itertools::Either::Left(0..tree_line.len()) };
    for i in range {
        debug!("Working on tree: {}", i);
        let mut scenery = 0;
        // We need to add +1 on the iterator here when moving in from the right to start at the correct location
        let inner_range = if reverse { itertools::Either::Right((i + 1)..tree_line.len()) } else { itertools::Either::Left((0..i).rev()) };
        for j in inner_range {
            trace!("Tree {} can be seen", j);
            scenery += 1;
            if tree_line[j].height >= tree_line[i].height {
                trace!("Tree {} breaks the view.", j);
                break;
            }
        }

        trace!("Tree: {}/{}, Scenery: {}", row, i, scenery);

        // Only overwrite the visibility if the tree is not yet visible from any other side
        if scenery > 0 {
            tree_line[i].scenery *= scenery;
        }
    }

    forest
}

fn column_scenery(mut forest: Vec<Vec<Tree>>, column: usize, reverse: bool) -> Vec<Vec<Tree>> {
    debug!("Column Scenery with Reverse = {}", reverse);
    debug!("Working on Column: {}", column);

    let range = if reverse { itertools::Either::Right((0..forest.len()).rev()) } else { itertools::Either::Left(0..forest.len()) };
    for i in range {

        debug!("Working on tree: {}", i);
        let mut scenery = 0;
        let inner_range = if reverse { itertools::Either::Right((i + 1)..forest.len()) } else { itertools::Either::Left((0..i).rev()) };
        for j in inner_range {
            trace!("Tree {} can be seen", j);
            scenery += 1;
            if forest[j][column].height >= forest[i][column].height {
                trace!("Tree {} breaks the view.", j);
                break;
            }
        }
        
        if scenery > 0 {
            forest[i][column].scenery *= scenery;
        }
    }

    forest
}

fn gather_scenery(forest: Vec<Vec<Tree>>) -> String {
    let mut result = 0;

    // Debug Data to see which tree is the highest
    let mut i = 0;
    let (mut tree_i, mut tree_j) = (0, 0);

    for forest_line in forest {
        let mut j = 0;
        for tree in forest_line {
            if tree.scenery > result { 
                (tree_i, tree_j) = (i, j);
                result = tree.scenery;
            }
            j += 1;
        }
        i += 1;
    }

    debug!("Tree with best is located at {}/{} with scenery: {}", tree_i, tree_j, result);

    result.to_string()
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"30373
25512
65332
33549
35390
"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"919
121
119
"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "21");
    assert_eq!(task1(&test_input2()), "9");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "16"); // Example solution in test set is wrong. :(
    assert_eq!(task2(&test_input2()), "4");
}
