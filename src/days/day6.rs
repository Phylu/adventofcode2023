use itertools::Itertools;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {

    return check_unique(content, 4);
}

fn task2(content: &String) -> String {

    return check_unique(content, 14);
}

fn check_unique(content: &String, unique_characters : usize) -> String {
    let char_buff : Vec<char> = content.chars().collect_vec();
    let mut counter = 0;
    
    for c in char_buff.windows(unique_characters) {
        if c.iter().all_unique() {
            return (counter + unique_characters).to_string();
        }

        counter += 1;
    }

    return String::from("");
}

#[test]
fn test_task1() {
    assert_eq!(task1(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), "5");
    assert_eq!(task1(&String::from("nppdvjthqldpwncqszvftbrmjlhg")), "6");
    assert_eq!(task1(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), "10");
    assert_eq!(task1(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), "11");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), "19");
    assert_eq!(task2(&String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), "23");
    assert_eq!(task2(&String::from("nppdvjthqldpwncqszvftbrmjlhg")), "23");
    assert_eq!(task2(&String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), "29");
    assert_eq!(task2(&String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), "26");
}
