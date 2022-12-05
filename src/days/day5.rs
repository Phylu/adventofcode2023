use itertools::Itertools;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {

    let (crates, starting_line) = prepare_crates(content);
    let crates = do_work(crates, content, starting_line);
    let result = gather_result(crates);

    return result;
}

fn task2(content: &String) -> String {

    let (crates, starting_line) = prepare_crates(content);
    let crates = do_work_1(crates, content, starting_line);
    let result = gather_result(crates);

    return result;
}

fn prepare_crates(content: &str) ->  (Vec<Vec<char>>, usize) {

    let mut crates : Vec<Vec<char>> = vec![];

    let mut line_number = 0;

    // Get number of crates
    for line in content.lines() {
        let mut char_line : Vec<char> = line.chars().collect();
        if char_line[1] == '1' {
            char_line.retain(|x| *x != ' ');
            crates = vec![vec![]; char_line.len()];

            break;
        }

        line_number += 1;
    }

    let lines_vector : Vec<&str> = content.lines().collect();
    for n in (0..line_number).rev() {

        let line = lines_vector[n];
        let line_vector : Vec<char> = line.chars().collect();
        for c in 0..crates.len() {
            let packet = line_vector[1+4*c]; 
            if packet != ' ' {
                crates[c].push(packet);
            }
        }

    }

    return (crates, line_number + 2);

}


fn do_work(mut crates: Vec<Vec<char>>, content: &str, starting_line: usize) -> Vec<Vec<char>> {

    let lines_vector : Vec<&str> = content.lines().collect_vec();

    for n in starting_line..lines_vector.len() {
        
        let line_vector : Vec<&str> = lines_vector[n].split(" ").collect();
        
        let counter: usize = line_vector[1].parse().unwrap();
        let from: usize = line_vector[3].parse().unwrap();
        let to: usize = line_vector[5].parse().unwrap();

        for _c in 0..counter {
            let element = crates[from - 1].pop().unwrap();
            crates[to - 1].push(element);
        }

    }

    return crates;
}

fn do_work_1(mut crates: Vec<Vec<char>>, content: &str, starting_line: usize) -> Vec<Vec<char>> {

    let lines_vector : Vec<&str> = content.lines().collect_vec();

    for n in starting_line..lines_vector.len() {
        
        let line_vector : Vec<&str> = lines_vector[n].split(" ").collect();
        
        let counter: usize = line_vector[1].parse().unwrap();
        let from: usize = line_vector[3].parse().unwrap();
        let to: usize = line_vector[5].parse().unwrap();

        let new_len = crates[from - 1].len() - counter;
        let mut elements : Vec<char> = crates[from - 1].drain(new_len..).collect();
        crates[to - 1].append(&mut elements);

    }

    return crates;
}

fn gather_result(crates: Vec<Vec<char>>) -> String {
    let mut result = "".to_owned();

    for c in 0..crates.len() {
        result.push_str(&crates[c][crates[c].len()-1].to_string());
    }

    return result.to_string();
}


#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input/5.txt").unwrap(); 
    assert_eq!(task1(&content), "QMBMJDFTD");
}

#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input/5.txt").unwrap(); 
    assert_eq!(task2(&content), "NBTVTJNFJ");
}
