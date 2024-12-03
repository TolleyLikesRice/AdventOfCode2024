use std::{fs::File, io::Read};

use regex::Regex;

pub fn main() {
    let mut file = File::open("inputs/day3.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut total = 0;

    for command in regex.captures_iter(&input) {
        total += command[1].parse::<i32>().unwrap() * command[2].parse::<i32>().unwrap();
    }

    println!("{}", total);

    let regex2 = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut enabled = true;
    total = 0;

    for command in regex2.captures_iter(&input) {
        if &command[0] == "do()" {
            enabled = true;
        } else if &command[0] == "don't()" {
            enabled = false;
        } else {
            if enabled {
                total += command[1].parse::<i32>().unwrap() * command[2].parse::<i32>().unwrap();
            }
        }
    }

    println!("{}", total);
}
