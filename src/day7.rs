use std::{fs::File, io::Read};

pub fn main() {
    let mut file = File::open("inputs/day7.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let mut sum: usize = 0;

    for problem in input.lines() {
        let mut problem = problem.split_whitespace().collect::<Vec<&str>>();

        let target_result = problem[0]
            .chars()
            .take(problem[0].len() - 1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        problem.remove(0);

        // parse problem into vector of numbers
        let problem = problem
            .iter()
            .map(|x| x.parse::<usize>().expect(&format!("Unable to parse {}", x)))
            .collect::<Vec<usize>>();

        // Part 1:
        //let operator_combinations =
        //get_possible_operator_combinations(problem.len() - 1, vec!['+', '*']);

        // Part 2:
        let operator_combinations =
            get_possible_operator_combinations(problem.len() - 1, vec!['+', '*', '|']);
        // Yes, the problem technically asks for the operator to be "||", but I can't be arsed to use strings since I already had it as chars.

        for combination in operator_combinations {
            let mut running_total: usize = problem[0];
            // For each space for an operator
            for i in 1..(problem.len()) {
                // Apply the operator to the running total
                match combination[i - 1] {
                    '+' => running_total += problem[i],
                    '*' => running_total *= problem[i],
                    '|' => {
                        let mut current_total = running_total.to_string();
                        let n = problem[i].to_string();

                        current_total.push_str(&n);

                        running_total = current_total
                            .parse::<usize>()
                            .expect("Failed to parse after concat");
                    }
                    _ => panic!("Invalid operator"),
                }
            }

            if running_total == target_result {
                sum += target_result;
                break;
            }
        }
    }

    println!("Result: {}", sum);
}

fn get_possible_operator_combinations(
    number_of_operators: usize,
    operators: Vec<char>,
) -> Vec<Vec<char>> {
    let mut combinations = vec![];

    for i in 0..(operators.len().pow(number_of_operators as u32)) {
        let mut combination = vec![];
        for j in 0..number_of_operators {
            combination
                .push(operators[(i / operators.len().pow(j as u32)) as usize % operators.len()]);
        }
        combinations.push(combination);
    }

    combinations
}
