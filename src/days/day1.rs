pub fn tasks(content: &String) {
    task1(content);
    task2(content);
}

fn task1(content: &String) {

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

}

fn task2(content: &String) {

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

    println!("{}", sum)

}
