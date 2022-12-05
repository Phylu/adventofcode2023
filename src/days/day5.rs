use itertools::Itertools;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {

    let (crates, starting_line) = prepare_crates(content);
    let crates = do_work(crates, content, starting_line, false);
    let result = gather_result(crates);

    return result;
}

fn task2(content: &String) -> String {

    let (crates, starting_line) = prepare_crates(content);
    let crates = do_work(crates, content, starting_line, true);
    let result = gather_result(crates);

    return result;
}

fn prepare_crates(content: &str) ->  (Vec<String>, usize) {

    // Set up crates vector
    let mut crates : Vec<String> = vec![];

    // Storage variable for storing the line in which the number of crates are defined
    let mut line_number = 0;

    // Get number of crates
    for line in content.lines() {

        let mut crates_line = String::from(line);
        crates_line = String::from(crates_line.trim());
        
        // Check if the 1st character in the string is a literal 1 after stripping whitespaces.
        // This denotes that in this line the number of crates is mentioned
        if crates_line.get(0..1).unwrap() == "1" {
            
            let (_, number_of_crates) = crates_line.rsplit_once(' ').unwrap();      
            crates = vec![String::from(""); number_of_crates.parse().unwrap()];

            break;
        }

        line_number += 1;
    }

    let lines_vector : Vec<&str> = content.lines().collect();
    // Iterate back from the bottom number of crates to the start of the file
    for n in (0..line_number).rev() {

        let line = lines_vector[n];
        let line_vector : Vec<char> = line.chars().collect();
        // Iterate over the number of crates that exist
        for c in 0..crates.len() {
            // The position of a single char in a crate is:
            // + 1 due to the leading whitespace
            // + 4 * c (crate number) due to the "] [" separator plus the character of the crate itself 
            let packet = line_vector[1+4*c]; 
            // If there are no more characters for the crate we just skip it
            if packet != ' ' {
                crates[c].push(packet);
            }
        }

    }

    return (crates, line_number + 2);

}


fn do_work(mut crates: Vec<String>, content: &str, starting_line: usize, reverse: bool) -> Vec<String> {

    // Get all lines as a vector to simplify starting at the starting line
    let lines_vector : Vec<&str> = content.lines().collect_vec();
    for n in starting_line..lines_vector.len() {
        
        // Split our command into its parts
        let line_vector : Vec<&str> = lines_vector[n].split(" ").collect();
        let counter: usize = line_vector[1].parse().unwrap();
        let from: usize = line_vector[3].parse().unwrap();
        let to: usize = line_vector[5].parse().unwrap();

        // Task 1 simply pup and push
        if !reverse {
            // Pop and Push our crates
            for _c in 0..counter {
                let element = crates[from - 1].pop().unwrap();
                crates[to - 1].push(element);
            }
        // Task 2 reverse order by taking all crates at a time
        } else {
            let from_length = crates[from - 1].len();
            // Pull crates from one stack
            let elements = crates[from - 1].split_off(from_length - counter);
            // Push onto the next stack
            crates[to - 1].push_str(elements.as_str());
        }


    }

    return crates;
}


fn gather_result(mut crates: Vec<String>) -> String {
    let mut result = String::from("");

    for c in 0..crates.len() {
        result.push(crates[c].pop().unwrap());
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
