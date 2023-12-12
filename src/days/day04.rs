pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let vec = prepare_input_1(content);
    let result: i32 = vec.iter().sum();

    return result.to_string();
}

fn task2(content: &String) -> String {
    let vec = prepare_input_2(content);
    let result: i32 = vec.iter().sum();

   return result.to_string();
}

fn prepare_input_1(content: &String) -> Vec<i32> {

    let mut vec = Vec::new();

    for line in content.lines() {

        if line != "" {

            let segments: Vec<&str> = line.split(":").collect();
            let game: Vec<&str> = segments[1].split("|").collect();

            let winning: Vec<&str> = game[0].split(" ").collect();
            let own: Vec<&str> = game[1].split(" ").collect();

            let mut points = 0;

            // Check each of our numbers if we get points for it
            for current in own {

                if current != "" {
                    if winning.contains(&current) {

                        if points == 0 {
                            points = 1;
                        } else {
                            points *= 2;
                        }
                        
                    }
                    
                }
                
            }
            
            vec.push(format!("{}", points).parse().unwrap());
            
        }
    }
    
    return vec;
}

fn prepare_input_2(content: &String) -> Vec<i32> {

    let cards_count = content.lines().count();
    let mut vec = vec![1; cards_count];

    let mut current = 0;

    for line in content.lines() {

        if line != "" {

            let segments: Vec<&str> = line.split(":").collect();
            let game: Vec<&str> = segments[1].split("|").collect();

            let winning: Vec<&str> = game[0].split(" ").collect();
            let own: Vec<&str> = game[1].split(" ").collect();

            let mut new_cards = 0;

            // Check each of our numbers if we get points for it
            for current in own {
                if current != "" {
                    if winning.contains(&current) {
                        new_cards += 1;
                    }
                }
            }

            let current_cards = vec[current];

            // Add new cards
            for n in current+1..current+new_cards+1 {
                println!("Adding Card {}, {} times", n+1, current_cards);
                vec[n] += current_cards;
            }

            current += 1;
        }
    }
    
    
    println!("{:?}", vec);
    return vec;
}

#[test]
fn test_task1() {
    let content = std::fs::read_to_string("input_test/4.txt").unwrap(); 
    assert_eq!(task1(&content), "13");
}


#[test]
fn test_task2() {
    let content = std::fs::read_to_string("input_test/4.txt").unwrap(); 
    assert_eq!(task2(&content), "30");
}
