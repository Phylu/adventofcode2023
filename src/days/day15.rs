use std::{vec, cmp};
use log::{debug, info};
use regex::Regex;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content, GOAL_ROW);
    let result2 = task2(content, MAX_ROW);
    return (result1, result2);
}

const GOAL_ROW: i32 = 2000000;
const MAX_ROW: i32 = 4000000;

#[derive(Debug, Clone, Copy)]
struct Pos (i32, i32);

fn task1(content: &String, goal_row: i32) -> String {

    let sensors = parse_input(content);
    let mut no_beacons : Vec<Pos> = vec![];

    for sensor in sensors {
        let mut p = positions_in_line(sensor.0, sensor.1, sensor.4, goal_row);
        no_beacons.append(&mut p);
    }

    no_beacons = merge_intervals(&mut no_beacons);
    debug!("No Beacons: {:?}", no_beacons);

    let result = no_beacons[0].1 - no_beacons[0].0;

    result.to_string()
}

fn task2(content: &String, max_row: i32) -> String {

    let sensors = parse_input(content);
    let mut distress_beacon = Pos(0, 0);

    for i in 0..max_row {

        let mut no_beacons : Vec<Pos> = vec![];
    
        for sensor in &sensors {
            let mut p = positions_in_line(sensor.0, sensor.1, sensor.4, i);
            no_beacons.append(&mut p);
        }
    
        no_beacons = merge_intervals(&mut no_beacons);
        debug!("No Beacons for row {}: {:?}", i, no_beacons);
    
        // We have two intervals that are not adjacent
        if no_beacons.len() == 2 {
            info!("There is an empty beacon spot in line: {}", i);
            distress_beacon = Pos(no_beacons[0].1 + 1, i);
            break
        }
    }

    info!("Distress beacon is located at: {:?}", distress_beacon);
    let result: i64 = distress_beacon.0 as i64 * 4000000 + distress_beacon.1 as i64;
    result.to_string()
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

fn positions_in_line(sx: i32, sy: i32, sensor_distance: i32, goal_row: i32) -> Vec<Pos> {
    let steps_in_row = sensor_distance - (sy - goal_row).abs();
    if steps_in_row >= 0 {
        return vec![Pos(sx - steps_in_row, sx + steps_in_row)];
    }
    vec![]
}

fn merge_intervals(intervals: &mut Vec<Pos>) -> Vec<Pos> {
    intervals.sort_by(|a,b| a.0.cmp(&b.0));
    
    let mut result: Vec<Pos> = vec![];
    result.push(intervals.remove(0));

    for current in intervals {
        let j = result.len() - 1;
        // Current starting inside or next to the rightmost interval of our results
        if current.0 <= result[j].1 + 1 {
            result[j].1 = cmp::max(current.1, result[j].1);
        } else {
            result.push(*current);
        }
    }

    result
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
    assert_eq!(task2(&test_input(), 20), "56000011");
}
