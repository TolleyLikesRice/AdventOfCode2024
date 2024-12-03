use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let mut file = File::open("inputs/day1.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let parts = line.split_whitespace();
        let num1 = parts.clone().next().unwrap().parse::<i32>().unwrap();
        let num2 = parts.clone().last().unwrap().parse::<i32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    list1.sort();
    list2.sort();

    let mut counter = 0;

    for i in 0..list1.len() {
        counter += (list1[i] - list2[i]).abs();
    }

    println!("{}", counter);

    // Part 2

    let mut similarity_score = 0;

    for num in &list1 {
        let count1 = list2.iter().filter(|&n| n == num).count();

        similarity_score += count1 as i32 * num;
    }

    println!("{}", similarity_score);
}
