use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let mut file = File::open("inputs/day2.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let mut safe_count = 0;

    for report in input.lines() {
        let readings: Vec<&str> = report.split_whitespace().collect();
        let is_safe = test_report(readings);

        if is_safe {
            safe_count += 1;
        }
    }

    println!("{}", safe_count);

    // Part 2
    let mut safe_count2 = 0;

    for report in input.lines() {
        let readings: Vec<&str> = report.split_whitespace().collect();
        let mut is_safe = false;

        for i in 0..readings.len() {
            let mut readings2 = readings.clone();
            readings2.remove(i);

            let is_this_safe = test_report(readings2);

            if is_this_safe {
                is_safe = true;
            }
        }

        if is_safe {
            safe_count2 += 1;
        }
    }

    println!("{}", safe_count2);
}

fn test_report(readings: Vec<&str>) -> bool {
    let mut is_safe = true;

    let mut increasing_count = 0;
    let mut decreasing_count = 0;

    for i in 0..readings.len() {
        if i != 0 {
            let diff =
                readings[i].parse::<i32>().unwrap() - readings[i - 1].parse::<i32>().unwrap();

            if diff > 0 {
                increasing_count += 1;
            } else if diff < 0 {
                decreasing_count += 1;
            }
            if diff == 0 || diff.abs() > 3 {
                is_safe = false;
            }
        }
    }

    if increasing_count != 0 && decreasing_count != 0 {
        is_safe = false;
    }

    is_safe
}
