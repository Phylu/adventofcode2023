use std::vec;

use log::{debug, error};
use radix::RadixNum;

pub fn tasks(content: &String) -> (String, String) {
    let result1 = task1(content);
    let result2 = task2(content);
    return (result1, result2);
}

fn task1(content: &String) -> String {
    let mut fuel_sum = 0;
    for line in content.lines() {
        if line.len() > 0 {
            fuel_sum += snafu2decimal(line);
        }
    }

    decimal2snafu(fuel_sum)
}

fn task2(content: &String) -> String {
    String::from("")
}

fn snafu2decimal(snafu: &str) -> i64 {
    let five: i64 = 5;
    let mut decimal: i64 = 0;
    let mut i = 0;

    for c in snafu.chars().rev() {
        let base: i64 = five.pow(i);

        let factor: i64 = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("This should never happen!"),
        };
        decimal += base * factor;
        i += 1;
    }

    decimal
}

fn decimal2snafu(decimal: i64) -> String {
    let mut snafu = "".to_string();
    
    let mut radix = RadixNum::from_str(&decimal.to_string(), 10).unwrap();
    radix = radix.with_radix(5).unwrap();

    debug!("Dec: {}, Radix: {}", decimal, radix);

    // Manually move this into a reversed vector for easier handlung
    let digits_iter = radix.digits();
    let mut reversed_digits : Vec<u32> = vec![];
    for d in digits_iter {
        reversed_digits.insert(0, d.to_digit(10).unwrap());
    }

    debug!("{:?}", reversed_digits);
    let mut final_carry_on = false;
    for i in 0..reversed_digits.len() {
        let digit = reversed_digits[i];

        // Add the carry if needed
        if digit >= 3 {
            if i < reversed_digits.len() - 1 {
                reversed_digits[i + 1] += 1;
            } else {
                final_carry_on = true;
            }
        }

        let current_snafu = match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            5 => '0',
            _ => panic!("This should never happen!"),
        };
        
        snafu.insert(0, current_snafu);
    }

    if final_carry_on {
        snafu.insert(0, '1');
    }
    
    snafu
}


#[cfg(test)]
fn test_input() -> String {
    String::from(r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#)
}

#[cfg(test)]
fn test_input2() -> String {
    String::from(r#"

"#)
}

#[test]
fn test_snafu2decimal() {
    assert_eq!(1, snafu2decimal("1"));
    assert_eq!(2, snafu2decimal("2"));
    assert_eq!(3, snafu2decimal("1="));
    assert_eq!(4, snafu2decimal("1-"));
    assert_eq!(5, snafu2decimal("10"));
    assert_eq!(6, snafu2decimal("11"));
    assert_eq!(7, snafu2decimal("12"));
    assert_eq!(8, snafu2decimal("2="));
    assert_eq!(9, snafu2decimal("2-"));
    assert_eq!(10, snafu2decimal("20"));
    assert_eq!(15, snafu2decimal("1=0"));
    assert_eq!(20, snafu2decimal("1-0"));
    assert_eq!(2022, snafu2decimal("1=11-2"));
    assert_eq!(12345, snafu2decimal("1-0---0"));
    assert_eq!(314159265, snafu2decimal("1121-1110-1=0"));
}

#[test]
fn test_decimal2snafu() {
    assert_eq!(decimal2snafu(1), "1".to_string());
    assert_eq!(decimal2snafu(2), "2".to_string());
    assert_eq!(decimal2snafu(3), "1=".to_string());
    assert_eq!(decimal2snafu(4), "1-".to_string());
    assert_eq!(decimal2snafu(5), "10".to_string());
    assert_eq!(decimal2snafu(6), "11".to_string());
    assert_eq!(decimal2snafu(7), "12".to_string());
    assert_eq!(decimal2snafu(8), "2=".to_string());
    assert_eq!(decimal2snafu(9), "2-".to_string());
    assert_eq!(decimal2snafu(10), "20".to_string());
    assert_eq!(decimal2snafu(15), "1=0".to_string());
    assert_eq!(decimal2snafu(20), "1-0".to_string());
    assert_eq!(decimal2snafu(2022), "1=11-2".to_string());
    assert_eq!(decimal2snafu(12345), "1-0---0".to_string());
    assert_eq!(decimal2snafu(314159265), "1121-1110-1=0".to_string());
}

#[test]
fn test_task1() {
    assert_eq!(task1(&test_input()), "2=-1=0");
}

#[test]
fn test_task2() {
    assert_eq!(task2(&test_input()), "");
    assert_eq!(task2(&test_input2()), "");
}
