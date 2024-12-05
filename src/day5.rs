use core::panic;
use std::{fs::File, io::Read};

pub fn main() {
    let mut file = File::open("inputs/day5.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let mut rules: Vec<Vec<i32>> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut parsing_rules = true;
    for line in input.lines() {
        if line == "" {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let rule: Vec<i32> = line.split("|").map(|x| x.parse().unwrap()).collect();
            rules.push(rule);
        } else {
            let update: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
            updates.push(update);
        }
    }

    let mut passing_updates: Vec<Vec<i32>> = Vec::new();
    let mut failed_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates {
        if validate_update(&update, &rules) {
            passing_updates.push(update.clone());
        } else {
            failed_updates.push(update.clone());
        }
    }

    let mut middle_page_sum = 0;

    for update in passing_updates {
        let middle_page = update[update.len() / 2];
        middle_page_sum += middle_page;
    }

    println!("{}", middle_page_sum);

    // Part 2
    let mut fixed_failed_updates: Vec<Vec<i32>> = Vec::new();

    for update in failed_updates {
        let fixed_update = fix_update(&update, &rules);

        fixed_failed_updates.push(fixed_update);
    }

    for update in &fixed_failed_updates {
        if !validate_update(&update, &rules) {
            panic!("Failed to fix update");
        }
    }

    middle_page_sum = 0;

    for update in fixed_failed_updates {
        let middle_page = update[update.len() / 2];
        middle_page_sum += middle_page;
    }

    println!("{}", middle_page_sum);
}

fn fetch_related_rules(rules: &Vec<Vec<i32>>, page_num: i32) -> Vec<Vec<i32>> {
    let mut related_rules: Vec<Vec<i32>> = Vec::new();
    for rule in rules {
        if rule[0] == page_num {
            related_rules.push(rule.clone());
        }
    }
    related_rules
}

fn validate_update(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let mut update_passing = true;

    for page in update.clone() {
        let related_rules = fetch_related_rules(&rules, page);
        let relevant_rules: Vec<Vec<i32>> = related_rules
            .iter()
            .filter(|rule| update.contains(&rule[1]))
            .map(|rule| rule.clone())
            .collect();

        for rule in relevant_rules {
            // find index of page in update
            let page_index = update.iter().position(|&x| x == page).unwrap();
            // find index of rule[1] in update
            let rule_index = update.iter().position(|&x| x == rule[1]).unwrap();

            if page_index > rule_index {
                update_passing = false;
                //println!("Failed on page {} with rule {:?}", page, rule);
                break;
            }
        }
    }

    update_passing
}

fn fix_update(update: &Vec<i32>, rules: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut fixed_update = update.clone();

    for page in update.clone() {
        let related_rules = fetch_related_rules(&rules, page);
        let relevant_rules: Vec<Vec<i32>> = related_rules
            .iter()
            .filter(|rule| update.contains(&rule[1]))
            .map(|rule| rule.clone())
            .collect();

        for rule in relevant_rules {
            // find index of page in update
            let page_index = fixed_update.iter().position(|&x| x == page).unwrap();
            // find index of rule[1] in update
            let rule_index = fixed_update.iter().position(|&x| x == rule[1]).unwrap();

            if page_index > rule_index {
                fixed_update.remove(page_index);
                fixed_update.insert(rule_index, page);
            }
        }
    }

    fixed_update
}
