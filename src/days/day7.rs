use std::{cell::RefCell, rc::Rc};
use regex::Regex;

const COMMAND: &str = r"^\$\s(?P<command>\S+)\s?(?P<path>\S+)?";
const FILE: &str = r"^(?P<size>\d+)\s(?P<filename>\S+)";

enum COMMANDS {
    CD,
}

impl COMMANDS {
    fn equals(&self, command: &str) -> bool {
        match self {
            COMMANDS::CD => if command == "cd" { true } else { false }
        }

    }
}

struct Directory {
    _name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Rc<RefCell<Directory>>>,
    size: u32,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            _name: name,
            parent: None,
            children: vec![],
            size: 0,
        }
    }
}

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let parent = prepare_input(content);
    get_small_dirs(&parent).to_string()
}

fn task2(content: &String) -> String {
    let parent = prepare_input(content);
    let free = 70000000 - parent.borrow().size;
    let minimum = 30000000 - free;
    get_smallest_deletable_dir(&parent, minimum).to_string()
}

fn prepare_input(content: &String) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::new(String::from("/"))));
    let mut current_dir = Rc::clone(&root);

    current_dir = read_input(content, current_dir);
    finalize_input(current_dir);

    root
}

fn read_input(content: &String, mut current_dir: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
    
    for line in content.lines() {

        let command_re : Regex = Regex::new(COMMAND).unwrap();
        let file_re : Regex = Regex::new(FILE).unwrap();
        
        // Handling a command
        if command_re.is_match(line) {
            let captures = command_re.captures(line).unwrap();
            let command = String::from(&captures["command"]);

            if COMMANDS::CD.equals(&command) {
                let path = String::from(&captures["path"]);

                if path == "/" {
                    // Empty, as parent directory has already been created
                } else if path == ".." {
                    current_dir = move_up(Rc::clone(&current_dir));
                } else {
                    current_dir = move_down(Rc::clone(&current_dir), path);
                }

            }

        } else if file_re.is_match(line) {
            let captures = file_re.captures(line).unwrap();
            let size: u32 = String::from(&captures["size"]).parse().unwrap();

            current_dir.borrow_mut().size += size;
        }
    }

    current_dir
}

fn finalize_input(mut current_dir: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {

    // Move up and store sizes
    loop {
     
        let current_clone = Rc::clone(&current_dir);
        let current_size = current_dir.borrow().size;

        current_dir = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
        current_dir.borrow_mut().size += current_size;

        if current_dir.borrow().parent.is_none() {
            break
        }
    }

    current_dir

}

fn move_up(mut current_dir: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
    let current_clone = Rc::clone(&current_dir);
    let current_size = current_dir.borrow().size;

    current_dir = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
    current_dir.borrow_mut().size += current_size;

    current_dir
}

fn move_down(current_dir: Rc<RefCell<Directory>>, path: String) -> Rc<RefCell<Directory>> {
    let next_dir = Rc::new(RefCell::new(Directory::new(path)));
    current_dir.borrow_mut().children.push(Rc::clone(&next_dir));
    {
        let mut mut_next = next_dir.borrow_mut();
        mut_next.parent = Some(Rc::clone(&current_dir));
    }

    next_dir
}

fn get_small_dirs(parent: &Rc<RefCell<Directory>>) -> u32 {
    let mut result : u32 = 0;

    result += get_small_enough(&parent);
    for child in &parent.borrow().children {
        result += get_small_dirs(child);
    }

    result
}

fn get_small_enough(dir: &Rc<RefCell<Directory>>) -> u32 {
    if dir.borrow().size <= 100000 { dir.borrow().size } else { 0 }
}

fn get_smallest_deletable_dir(parent: &Rc<RefCell<Directory>>, minimum: u32) -> u32 {
    let mut result : u32 = u32::MAX;

    let current_dir_size = get_large_enough(&parent, minimum);

    if current_dir_size < result {
        result = current_dir_size
    }

    for child in &parent.borrow().children {
        let child_size = get_smallest_deletable_dir(child, minimum);

        if child_size < result {
            result = child_size
        }
    }

    result
}

fn get_large_enough(dir: &Rc<RefCell<Directory>>, minimum: u32) -> u32 {
    if dir.borrow().size >= minimum { dir.borrow().size } else { u32::MAX }
}


#[cfg(test)]
fn test_input() -> String {
    String::from(r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "95437");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "24933642");
}
