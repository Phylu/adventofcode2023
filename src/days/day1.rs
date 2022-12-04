pub fn tasks(content: &String) {
    task1(content);
    task2(content);
}

fn task1(content: &String) -> i32 {

    let mut max = 0;
    let mut current = 0;

    for line in content.lines() {
        if line == "" {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            let number : i32 = line.parse().unwrap();
            current += number;
        }
    }

    if current > max {
        max = current;
    }  

    println!("{}", max);
    return max;

}

fn task2(content: &String) -> i32 {

    let mut vec = Vec::new();
    let mut current = 0;

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

    let top3 = &vec[vec.len()-3..vec.len()];
    let sum: i32 = top3.iter().sum();

    println!("{}", sum);
    return sum;

}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input/1.txt").unwrap(); 
    assert_eq!(task1(&content), 69289);
}

#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input/1.txt").unwrap(); 
    assert_eq!(task2(&content), 205615);
}