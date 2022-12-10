use log::{debug, warn};

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let state = parse_input(content);
    state.strength.to_string()
}

fn task2(content: &String) -> String {
    let state = draw_input(content);
    format!("\n{}", state.pcr)
}

const CYCLES : [i32; 6] = [20, 60, 100, 140, 180, 220];

struct State {
    tick: i32,
    x: i32,
    strength: i32,
    pcr: String,
}

impl State {
    fn tick(mut self, draw: bool) -> State {
        self.tick += 1;

        // Task 1
        if CYCLES.contains(&self.tick) {
            debug!("At cycle {}, register x is at {} so it adds {} to the signal strengt", self.tick, self.x, self.tick * self.x);
            self.strength += self.tick * self.x;
        }

        // Task 2
        if draw {
            let pcr_pos = (self.tick - 1) % 40;

            if (pcr_pos - self.x).abs() <= 1 {
                self.pcr += "#";
            } else {
                self.pcr += ".";
            }

            if pcr_pos == 39 {
                self.pcr += "\n"
            }
        }

        self
    }

    fn inc(mut self, inc: i32) -> State {
        self.x += inc;
        self
    }
}

struct Command {
    ticks: i32,
    inc: i32,
    draw: bool,
}

impl Command {
    fn new(ticks: i32, inc: i32, draw: bool) -> Command {
        Command {
            ticks: ticks,
            inc: inc,
            draw: draw,
        }
    }

    fn run(self, state: State) -> State {
        // Let's move the state into the function for this to work properly
        let mut this_state = state;

        for _ in 0..self.ticks {
            this_state = this_state.tick(self.draw);
        }
        
        this_state = this_state.inc(self.inc);
        this_state
    }
}

fn parse_input(content: &String) -> State {
    let mut state: State = State { tick: 0, x: 1, strength: 0, pcr: String::from("") };

    for line in content.lines() {
        let cmdline: Vec<&str> = line.split(" ").collect();
        let mut cmd = Command::new(0, 0, false);
        match cmdline[0] {
            "noop" => cmd.ticks = 1,
            "addx" => {
                cmd.ticks = 2;
                cmd.inc = cmdline[1].parse().unwrap();
            }
            _ => warn!("This should never happen!")
        }
        state = cmd.run(state);
    }
    
    let finalcmd = Command::new(1, 0, false);
    state = finalcmd.run(state);

    state
}

fn draw_input(content: &String) -> State {
    let mut state: State = State { tick: 0, x: 1, strength: 0, pcr: String::from("") };

    for line in content.lines() {
        let cmdline: Vec<&str> = line.split(" ").collect();
        let mut cmd = Command::new(0, 0, true);
        match cmdline[0] {
            "noop" => cmd.ticks = 1,
            "addx" => {
                cmd.ticks = 2;
                cmd.inc = cmdline[1].parse().unwrap();
            }
            _ => warn!("This should never happen!")
        }
        state = cmd.run(state);
    }

    state
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "13140");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#);
}
