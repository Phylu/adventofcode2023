pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {

    let vec = prepare_input(content);

    return vec[vec.len()-1].to_string();
}

fn task2(content: &String) -> String {

    let vec = prepare_input(content);

    let top3 = &vec[vec.len()-3..vec.len()];
    let sum: i32 = top3.iter().sum();

    return sum.to_string();

}

fn prepare_input(content: &String) -> Vec<i32> {

    let mut current = 0;
    let mut vec = Vec::new();

    for line in content.lines() {
        if line == "" {
            vec.push(current);
            current = 0;
        } else {
            let number : i32 = line.parse().unwrap();
            current += number;
        }
    }
    vec.push(current);
    
    vec.sort();
    return vec;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input/1.txt").unwrap(); 
    assert_eq!(task1(&content), "69289");
}

#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input/1.txt").unwrap(); 
    assert_eq!(task2(&content), "205615");
}
