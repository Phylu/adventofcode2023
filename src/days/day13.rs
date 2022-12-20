use std::cmp;
use std::cmp::Ordering;
use log::debug;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let mut result = 0;
    let mut i = 0;

    for input in content.split("\n\n") {
        i += 1;
        let (input_1, input_2) = input.split_once("\n").unwrap();

        debug!("\nTesting Nr. {}:\n{}\n{}", i, input_1, input_2);
        let thing_1 = create_thing(String::from(input_1));
        let thing_2 = create_thing(String::from(input_2.trim_end_matches("\n")));

        if correct_order(&thing_1, &thing_2) == Ordering::Less {
            result += i;
            debug!("{} is in correct order.", i);
        }

    }

    result.to_string()
}

fn task2(content: &String) -> String {
    let mut packages : Vec<Thing> = vec![];

    for line in content.lines() {
        if line.len() == 0 {
            continue
        }
        packages.push(create_thing(String::from(line)));
    }
    packages.push(create_thing(String::from("[[2]]")));
    packages.push(create_thing(String::from("[[6]]")));

    packages.sort_by(|a, b| correct_order(a, b));

    decoder_keys(packages).to_string()
}

#[derive(PartialEq, Clone, Debug)]
enum Thing {
    Number(i32),
    List(Vec<Box<Thing>>),
}

fn create_thing(mut line: String) -> Thing {
    // No list, so we have a number here;
    if !line.contains("[") {
        return Thing::Number(line.parse().unwrap());
    }

    // We have a list
    let mut v: Vec<Box<Thing>> = vec![];

    // Remove the outermost list
    if line.chars().next().unwrap() == '[' && line.chars().last().unwrap() == ']' {
        line.remove(0);
        line.pop();
    }

    let mut bracket_counter = 0;
    let mut current_thing = String::from("");
    for c in line.chars() {
        // Push something to the vector when we have a outmost element
        if bracket_counter == 0 && c == ',' {
            v.push(Box::new(create_thing(current_thing.clone())));
            // Reset the thing after we have pushed it
            current_thing = String::from("");
        } else {
            current_thing.push(c);
            if c == '[' {
                bracket_counter += 1;
            } else if c == ']' {
                bracket_counter -= 1;
            }
        }
    }
    // Push the last item as well
    if current_thing.len() > 0 {
        v.push(Box::new(create_thing(current_thing.clone())));
    }

    Thing::List(v)
}

// Returns +1 if first element is smaller (correct order), 0 if equal, -1 if the first element is bigger
fn correct_order(first: &Thing, second: &Thing) -> Ordering {
    //println!("Comparing: {:?} & {:?}", first, second);

    match (first, second) {
        (Thing::Number(i), Thing::Number(j)) => {
            //println!("Comparing Numbers: {} & {}", i, j);
            if i < j {
                Ordering::Less
            } else if i == j {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        },
        (Thing::List(i), Thing::List(j)) => {
            //println!("Comparing Lists: {:?} & {:?}", i, j);
            if i.len() == 0 && j.len() == 0 {
                Ordering::Equal
            } else if i.len() == 0 { 
                //println!("j.len == 0: True");
                Ordering::Less
            } else if j.len() == 0 {
                //println!("i.len == 0: False");
                Ordering::Greater
            } else {

                // Check each list element individually.
                // If the order in one of the children is wrong, the whole order is wrong
                for pos in 0..cmp::min(i.len(), j.len()) {
                    let order = correct_order(&*i[pos].clone(), &*j[pos].clone());
                    if order != Ordering::Equal {
                        return order;
                    }
                }

                // If there are remaining elements in the first list, the order is wrong
                if i.len() < j.len() {
                    return Ordering::Less;
                } else if i.len() > j.len() {
                    return Ordering::Greater
                }

                // Otherwise the order of the children is correct
                Ordering::Equal
            }
        },
        (Thing::Number(i), Thing::List(j)) => {
            //println!("Comparing Converted Number {} with List {:?}", i, j);
            let i_list: Thing = Thing::List(vec![Box::new(Thing::Number(*i))]);
            let j_list: Thing = Thing::List(j.to_vec());
            return correct_order(&i_list, &j_list);
        },
        (Thing::List(i), Thing::Number(j)) => {
            //println!("Comparing List {:?} with converted number {}", i, j);
            let i_list: Thing = Thing::List(i.to_vec());
            let j_list: Thing = Thing::List(vec![Box::new(Thing::Number(*j))]);
            return correct_order(&i_list, &j_list);
        },
    }

}

fn decoder_keys(packages: Vec<Thing>) -> usize {
    let mut result = 1;
    for i in 0..packages.len() {
        let decoder_1 = create_thing(String::from("[[2]]"));
        let decoder_2 = create_thing(String::from("[[6]]"));
        debug!("{:?}", packages[i]);
        if packages[i] == decoder_1 || packages[i] == decoder_2 {
            result *= i + 1;
        }
    }

    result
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#)
}

#[test]
fn test_task1() {
   assert_eq!(task1(&test_input()), "13");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "140");
}

#[test]
fn test_create_easy_thing() {
    assert_eq!(create_thing(String::from("1")), Thing::Number(1));
    assert_eq!(create_thing(String::from("[]")), Thing::List(vec![]));
    assert_eq!(create_thing(String::from("[1]")), Thing::List(vec![Box::new(Thing::Number(1))]));
}

#[test]
fn test_create_big_thing() {
    let input = String::from("[1,[2]]");
    let thing = Thing::List(vec![Box::new(Thing::Number(1)), Box::new(Thing::List(vec![Box::new(Thing::Number(2))]))]);
    assert_eq!(create_thing(input), thing);
}

#[test]
fn test_create_strange_thing() {
    let input = String::from("[[[[3],4]]]");
    let thing = Thing::List(vec![
        Box::new(Thing::List(vec![
            Box::new(Thing::List(vec![
                Box::new(Thing::List(vec![
                    Box::new(Thing::Number(3))
                ]))
                ,
                Box::new(Thing::Number(4))
            ]))
        ]))
    ]);
    assert_eq!(create_thing(input), thing);
}

#[test]
fn test_crate_compare_strange_thing() {
    let t1 = create_thing(String::from("[[],[2,4,[[9,1,2,0,0]]],[[[0,8,2,5,5],2,1],[4]]]"));
    let t2 = create_thing(String::from("[[9,4],[],[[[4,7],4],[[10,8]],[5,[6,2],[8],[]],[[2,9,7,7],8],[[5,8,9,7,5]]],[2,[9,[],[8,8,3,7,2],1,[10,9,8,1,8]],[],5,0],[[],9,[5,0,[2,5,2,10,8],2],[[1,4,7,7],10],7]]"));
    assert_eq!(correct_order(&t1, &t2), Ordering::Less)
}

#[test]
fn test_compare_numbers() {
    assert_eq!(correct_order(&Thing::Number(1), &Thing::Number(1)), Ordering::Equal);
    assert_eq!(correct_order(&Thing::Number(10), &Thing::Number(20)), Ordering::Less);
    assert_eq!(correct_order(&Thing::Number(20), &Thing::Number(10)), Ordering::Greater);
}

#[test]
fn test_compare_lists() {
    assert_eq!(correct_order(&Thing::List(vec![Box::new(Thing::List(vec![]))]), &Thing::List(vec![])), Ordering::Greater);
    assert_eq!(correct_order(&Thing::List(vec![]), &Thing::List(vec![Box::new(Thing::List(vec![]))])), Ordering::Less);
    assert_eq!(correct_order(
        &Thing::List(vec![]),
        &Thing::List(vec![Box::new(Thing::Number(3))])
    ), Ordering::Less);
    assert_eq!(correct_order(
        &Thing::List(vec![Box::new(Thing::Number(9))])
        , 
        &Thing::List(vec![Box::new(Thing::List(vec![Box::new(Thing::Number(8)), Box::new(Thing::Number(7))]))])
    ), Ordering::Greater);
}
