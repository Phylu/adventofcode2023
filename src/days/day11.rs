pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operator: Operator,
    operation_arg: i64,
    test_divisor: i64,
    success_monkey: usize,
    failure_monkey: usize,
    inspections: i64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Square,
}

impl Monkey {

    fn operate_and_test(&mut self, worry_reduction: bool, common_divisor: i64) -> (i64, usize) {
        self.inspections += 1;
        let mut worry = self.items.pop().unwrap();

        if self.operator == Operator::Add {
            worry += self.operation_arg;
        } else if self.operator == Operator::Multiply {
            worry *= self.operation_arg;
        } else if self.operator == Operator::Square {
            worry *= worry;
        }

        if worry_reduction {
            worry /= 3;
        } else {
            worry %= common_divisor;
        }

        let recipient_monkey = if (worry % self.test_divisor) == 0 { self.success_monkey } else { self.failure_monkey};

        (worry, recipient_monkey)
    }
}


fn task1(content: &String) -> String {
    let monkeys: Vec<Monkey> = read_input(content);
    calculate_result(monkeys, 20, true, 1)
}

fn task2(content: &String) -> String {
    let monkeys: Vec<Monkey> = read_input(content);
    let mut common_divisor = 1;
    for i in 0..monkeys.len() {
        common_divisor *= monkeys[i].test_divisor;
    }
    calculate_result(monkeys, 10000, false, common_divisor)
}

fn read_input(content: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for monkey_block in content.split("\n\n") {

        let mut items: Vec<i64> = vec![];
        let mut operator: Operator = Operator::Add;
        let mut operation_arg = 1;
        let mut test_divisor = 1;
        let mut success_monkey = 1;
        let mut failure_monkey = 1;
        let inspections = 0;

        for line in monkey_block.lines() {

            if line.contains("Monkey") {
                continue;
            }

            let (title, value) = line.split_once(": ").unwrap();

            // Starting Items
            if title.contains("Starting items") {
                println!("Starting Items");
                for item in value.split(", ") {
                    let this_item : i64 = item.parse().unwrap()   ;
                    items.push(this_item);
                }
            }

            // Operation
            else if title.contains("Operation") { 
                if value.contains("old * old") {
                    operator = Operator::Square;
                } else if value.contains("old *") {
                    operator = Operator::Multiply;
                    operation_arg = value.split_once("old * ").unwrap().1.parse().unwrap();
                } else if value.contains("old +") {
                    operator = Operator::Add;
                    operation_arg = value.split_once("old + ").unwrap().1.parse().unwrap();
                }
            }

            // Test
            else if title.contains("Test") {
                test_divisor = value.split_once("divisible by ").unwrap().1.parse().unwrap();
            }

            // Success Monkey
            else if title.contains("If true") {
                success_monkey = value.split_once("throw to monkey ").unwrap().1.parse().unwrap();
            }

            // Failure Monkey
            else if title.contains("If false") {
                failure_monkey = value.split_once("throw to monkey ").unwrap().1.parse().unwrap();
            }

        }

        let monkey = Monkey { items, operator, operation_arg, test_divisor, success_monkey, failure_monkey, inspections};
        monkeys.push(monkey);
    }


    monkeys
}

fn calculate_result(mut monkeys: Vec<Monkey>, rounds: i32, worry_reduction: bool, common_divisor: i64) -> String {
        // 20 Rounds
        for _round in 0..rounds {

            for i in 0..monkeys.len() {
                while monkeys[i].items.len() > 0 {
                    let (worry, recipient_monkey) = monkeys[i].operate_and_test(worry_reduction, common_divisor);
                    monkeys[recipient_monkey].items.push(worry);
                }
            }
    
        }
    
        let mut inspections: Vec<i64> = vec![];
        for i in 0..monkeys.len() {
            inspections.push(monkeys[i].inspections);
        }
    
        inspections.sort();
        let result:i64 = inspections[inspections.len() - 2] * inspections[inspections.len() - 1];
    
        result.to_string()
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "10605");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "2713310158");
}
