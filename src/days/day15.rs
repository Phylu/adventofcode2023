use std::vec;

use itertools::Itertools;
use log::{debug, error};
use regex::Regex;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content, GOAL_ROW);
    let result2 = task2(content);
    return (result1, result2);
}

const GOAL_ROW: i32 = 2000000;

fn task1(content: &String, goal_row: i32) -> String {

    let sensors = parse_input(content);
    let mut no_beacons : Vec<i32> = vec![];
    let mut goal_row_beacons : Vec<i32> = vec![];

    for sensor in sensors {
        let mut p = positions_in_line(sensor.0, sensor.1, sensor.4, goal_row);
        no_beacons.append(&mut p);
        if sensor.3 == goal_row {
            goal_row_beacons.push(sensor.2)
        }
    }

    let mut result = 0;
    debug!("No Beacons: {:?}", no_beacons);
    debug!("Beacons: {:?}", goal_row_beacons);
    for x in no_beacons.iter().unique() {
        if !goal_row_beacons.contains(x) {
            result += 1;
        }
    }

    result.to_string()
}

fn task2(content: &String) -> String {

    let sensors = parse_input(content);
    let mut no_beacons : Vec<i32> = vec![];
    let mut goal_row_beacons : Vec<i32> = vec![];

    for sensor in sensors {
        let mut p = positions_in_line(sensor.0, sensor.1, sensor.4, goal_row);
        no_beacons.append(&mut p);
        if sensor.3 == goal_row {
            goal_row_beacons.push(sensor.2)
        }
    }

    let mut result = 0;
    debug!("No Beacons: {:?}", no_beacons);
    debug!("Beacons: {:?}", goal_row_beacons);
    for x in no_beacons.iter().unique() {
        if !goal_row_beacons.contains(x) {
            result += 1;
        }
    }

    //result.to_string()

    String::from("")
}

const SENSOR_STRING: &str = r"^Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)";

fn parse_input(content: &String) -> Vec<(i32, i32, i32, i32, i32)> {
    let mut sensors: Vec<(i32, i32, i32, i32, i32)> = vec![];

    let re : Regex = Regex::new(SENSOR_STRING).unwrap();
    for line in content.lines() {
        if re.is_match(line) {
            let captures = re.captures(line).unwrap();

            let sx : i32 = captures["sx"].parse().unwrap();
            let sy : i32 = captures["sy"].parse().unwrap();
            let bx : i32 = captures["bx"].parse().unwrap();
            let by : i32 = captures["by"].parse().unwrap();

            let sensor_distance = (sx - bx).abs() + (sy - by).abs();
            debug!("Sensor: {}/{}, Beacon: {}/{}, Distance: {}", sx, sy, bx, by, sensor_distance);

            sensors.push((sx, sy, bx, by, sensor_distance));
        }
    } 
    
    sensors
}

fn positions_in_line(sx: i32, sy: i32, sensor_distance: i32, goal_row: i32) -> Vec<i32> {
    let mut pos: Vec<i32> = vec![];

    let steps_in_row = sensor_distance - (sy - goal_row).abs();
    if steps_in_row >= 0 {
        for i in 0..steps_in_row + 1 {
            pos.push(sx + i);
            pos.push(sx - i);
        }
    }

    pos
}

#[cfg(test)]
fn test_input() -> String {
    String::from(r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#)
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input(), 10), "26");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "56000011");
}
