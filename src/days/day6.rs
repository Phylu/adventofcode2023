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
    let mut char_buff = String::from("");
    let mut counter = 0;

    for character in content.chars() {
        counter += 1;

        // First unique_characters need to be pushed in any case
        if char_buff.len() < unique_characters {
            char_buff.push(character);
        // Afterwards we remove the first character and check for duplicates
        } else {
            char_buff.remove(0);
            
            if !char_buff.contains(character) && !has_duplicates(&char_buff) {
                return counter.to_string();
            }

            char_buff.push(character);
        }
    }

    return String::from("");
}

fn has_duplicates(s: &str) -> bool {
    let curr = s;
    let mut checker = String::from(s);
    for c in curr.chars() {
        checker.remove(0).to_string();
        if checker.contains(c) {
            return true
        }
    }
    return false;
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
