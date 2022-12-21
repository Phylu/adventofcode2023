use std::{collections::HashMap, fmt};

use log::{debug, info};
use regex::Regex;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let (mut monkeys, uncalculated_monkeys) = read_input(content);

    monkeys = calculate_monkeys(monkeys, uncalculated_monkeys);

    let result = monkeys.get(&String::from("root")).unwrap().output.unwrap();

    result.to_string()
}

fn task2(content: &String) -> String {
    let (mut monkeys, _) = read_input(content);

    let root = String::from("root");
    let human = String::from("humn");

    // Setup the Human Monkey
    let human_monkey = monkeys.get_mut(&human).unwrap();
    human_monkey.output = None;

    // Setup the Root Monkey
    let root_monkey = monkeys.get(&root).unwrap();
    let root_first_op = root_monkey.first_operand.as_ref().unwrap().to_string();
    let root_second_op = root_monkey.second_operand.as_ref().unwrap().to_string();

    let mut uncalculated_monkeys_first = get_all_operands(monkeys.clone(), root_first_op.clone());
    let mut uncalculated_monkeys_second = get_all_operands(monkeys.clone(), root_second_op.clone());

    debug!("Uncalc 1: {:?}", uncalculated_monkeys_first);
    debug!("Uncalc 2: {:?}", uncalculated_monkeys_second);

    // Calculate first Root Tree
    if uncalculated_monkeys_first.contains(&String::from("humn")) {
        monkeys = calculate_monkeys(monkeys, uncalculated_monkeys_second.clone());

        let goal_result = monkeys.get(&root_second_op.clone()).unwrap().output.unwrap();
        monkeys.get_mut(&root_first_op.clone()).unwrap().output = Some(goal_result);
    } else {
        monkeys = calculate_monkeys(monkeys, uncalculated_monkeys_first.clone());

        let goal_result = monkeys.get(&root_first_op.clone()).unwrap().output.unwrap();
        monkeys.get_mut(&root_second_op.clone()).unwrap().output = Some(goal_result);
    }

    let first_result = monkeys
        .get(&root_second_op.clone())
        .unwrap()
        .output
        .unwrap();

        info!("First Result for {}: {}", root_second_op.to_string(), first_result);

    let mut all_uncalculated_monkeys: Vec<String> = vec![];
    all_uncalculated_monkeys.append(&mut uncalculated_monkeys_first);
    all_uncalculated_monkeys.append(&mut uncalculated_monkeys_second);
    
    while monkeys.get(&human).unwrap().output == None {
        monkeys = calculate_monkeys_down(monkeys, all_uncalculated_monkeys.clone());
        monkeys = calculate_monkeys(monkeys, all_uncalculated_monkeys.clone());
    }

    let result = monkeys.get(&human).unwrap().output.unwrap();
    result.to_string()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operation {
    fn operate(self, op1: i64, op2: i64) -> i64 {
        match self {
            Operation::Plus => op1 + op2,
            Operation::Minus => op1 - op2,
            Operation::Multiply => op1 * op2,
            Operation::Divide => op1 / op2,
        }
    }

    fn solve_left(self, op1: i64, result: i64) -> i64 {
        match self {
            Operation::Plus => result - op1,
            Operation::Minus => op1 - result,
            Operation::Multiply => result / op1,
            Operation::Divide => op1 / result,
        }
    }

    fn solve_right(self, op2: i64, result: i64) -> i64 {
        match self {
            Operation::Plus => result - op2,
            Operation::Minus => op2 + result,
            Operation::Multiply => result / op2,
            Operation::Divide => op2 * result,
        }
    }
}

impl fmt::Display for Operation {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut o = "";
        match &self {
            Operation::Plus => o = "+",
            Operation::Minus => o = "-",
            Operation::Multiply => o = "*",
            Operation::Divide => o = "/",
        }
        write!(f, "{}", o)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    first_operand: Option<String>,
    second_operand: Option<String>,
    operation: Option<Operation>,
    output: Option<i64>,
}

fn get_all_operands(monkeys: HashMap<String, Monkey>, monkey: String) -> Vec<String> {
    let mut operands: Vec<String> = vec![monkey.clone()];

    let current_monkey = monkeys.get(&monkey).unwrap();

    match current_monkey.first_operand.as_ref() {
        None => {}
        Some(x) => {
            operands.append(&mut get_all_operands(monkeys.clone(), x.to_string()));
        }
    }

    match current_monkey.second_operand.as_ref() {
        None => {}
        Some(x) => {
            operands.append(&mut get_all_operands(monkeys.clone(), x.to_string()));
        }
    }

    operands
}

fn read_input(content: &String) -> (HashMap<String, Monkey>, Vec<String>) {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    let mut uncalculated_monkeys: Vec<String> = vec![];

    const DIRECT_MONKEY: &str = r"^(?P<name>\S+):\s(?P<output>\d+)$";
    const INDIRECT_MONKEY: &str = r"^(?P<name>\S+):\s(?P<first_operand>[a-z]+)\s(?P<operation>[+\-*/])\s(?P<second_operand>[a-z]+)";

    let re_direct: Regex = Regex::new(DIRECT_MONKEY).unwrap();
    let re_indirect: Regex = Regex::new(INDIRECT_MONKEY).unwrap();

    for line in content.lines() {
        // Monkeys that have their number set
        if re_direct.is_match(line) {
            let captures = re_direct.captures(line).unwrap();

            let name = String::from(&captures["name"]);
            let output: i64 = captures["output"].parse().unwrap();

            let monkey = Monkey {
                first_operand: None,
                second_operand: None,
                operation: None,
                output: Some(output),
            };

            monkeys.insert(name.clone(), monkey);
        } else if re_indirect.is_match(line) {
            let captures = re_indirect.captures(line).unwrap();

            let name = String::from(&captures["name"]);
            let first_operand = String::from(&captures["first_operand"]);
            let second_operand = String::from(&captures["second_operand"]);
            let operation = match &captures["operation"] {
                "+" => Operation::Plus,
                "-" => Operation::Minus,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                &_ => panic!("This should never happen!"),
            };

            let monkey = Monkey {
                first_operand: Some(first_operand),
                second_operand: Some(second_operand),
                operation: Some(operation),
                output: None,
            };

            monkeys.insert(name.clone(), monkey);
            uncalculated_monkeys.push(name.clone());
        }
    }

    (monkeys, uncalculated_monkeys)
}

fn calculate_monkeys(
    mut monkeys: HashMap<String, Monkey>,
    mut uncalculated_monkeys: Vec<String>,
) -> HashMap<String, Monkey> {
    let mut no_change_rounds = 0;

    while uncalculated_monkeys.len() > 0 {
        // There is no more calculations that we can do, so we just return all the calculated monkeys that we have.
        if no_change_rounds > uncalculated_monkeys.len() {
            return monkeys;
        }

        let current_name = uncalculated_monkeys.remove(0);
        let current = monkeys.get(&current_name).unwrap();

        // Already calculated, so we skip here
        if current.output != None || current.first_operand == None || current.second_operand == None {
            continue;
        }

        let first_operand_name = current.first_operand.as_ref().unwrap();
        let second_operand_name = current.second_operand.as_ref().unwrap();

        let first_operand_monkey = monkeys.get(first_operand_name).unwrap();
        let second_operand_monkey = monkeys.get(second_operand_name).unwrap();

        if first_operand_monkey.output != None && second_operand_monkey.output != None {
            let op1 = first_operand_monkey.output.unwrap();
            let op2 = second_operand_monkey.output.unwrap();
            let output = current.operation.unwrap().operate(op1, op2);
            debug!(
                "{}:{} {} {}:{} = {}:{}",
                first_operand_name,
                op1,
                current.operation.unwrap(),
                second_operand_name,
                op2,
                current_name,
                output
            );

            let mut current = monkeys.get_mut(&current_name).unwrap();
            current.output = Some(output);

            no_change_rounds = 0;
        } else {
            uncalculated_monkeys.push(current_name);

            no_change_rounds += 1;
        }
    }

    monkeys
}

fn calculate_monkeys_down(
    mut monkeys: HashMap<String, Monkey>,
    mut uncalculated_monkeys: Vec<String>,
) -> HashMap<String, Monkey> {
    let mut no_change_rounds = 0;
    debug!("{:?}", uncalculated_monkeys);

    while uncalculated_monkeys.len() > 0 {
        // There is no more calculations that we can do, so we just return all the calculated monkeys that we have.
        if no_change_rounds > uncalculated_monkeys.len() {
            return monkeys;
        }

        let current_name = uncalculated_monkeys.remove(0);
        let current = monkeys.get(&current_name).unwrap();

        // Already calculated or does not need to calculate, so we skip here
        if current.first_operand == None || current.second_operand == None || current_name == "humn".to_string() {
            continue;
        }

        debug!("Working on the following monkeys: ");
        debug!("{}, {:?}", current_name, current);
        
        let first_operand_name = current.first_operand.as_ref().unwrap().to_string();
        let second_operand_name = current.second_operand.as_ref().unwrap().to_string();
        
        let first_operand_monkey = monkeys.get(&first_operand_name).unwrap();
        let second_operand_monkey = monkeys.get(&second_operand_name).unwrap();
        
        debug!("{}, {:?}", first_operand_name, first_operand_monkey);
        debug!("{}, {:?}", second_operand_name, second_operand_monkey);
        
        if current.output != None
            && first_operand_monkey.output != None
            && second_operand_monkey.output == None
        {
            let op1 = first_operand_monkey.output.unwrap();
            let output = current.output.unwrap();
            let op2 = current.operation.unwrap().solve_left(op1, output);
            debug!(
                "{}:{} {} {}:{} = {}:{}",
                first_operand_name,
                op1,
                current.operation.unwrap(),
                second_operand_name,
                op2,
                current_name,
                output
            );

            let mut second = monkeys.get_mut(&second_operand_name).unwrap();
            second.output = Some(op2);

            if second_operand_name == "humn" {
                info!("Human Calculated: {}", op2);
            }

            no_change_rounds = 0;
        } else if current.output != None
            && second_operand_monkey.output != None
            && first_operand_monkey.output == None
        {
            let op2 = second_operand_monkey.output.unwrap();
            let output = current.output.unwrap();
            let op1 = current.operation.unwrap().solve_right(op2, output);
            debug!(
                "{}:{} {} {}:{} = {}:{}",
                first_operand_name,
                op1,
                current.operation.unwrap(),
                second_operand_name,
                op2,
                current_name,
                output
            );

            let mut first = monkeys.get_mut(&first_operand_name).unwrap();
            first.output = Some(op1);

            if first_operand_name == "humn" {
                info!("Human Calculated: {}", op1);
            }

            no_change_rounds = 0;
        } else {
            uncalculated_monkeys.push(current_name);
            no_change_rounds += 1;
        }
    }

    monkeys
}

#[cfg(test)]
fn test_input() -> String {
    String::from(
        r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#,
    )
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "152");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "301");
}
