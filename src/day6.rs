use std::{collections::BTreeMap, fs::File, io::Read};

pub fn main() {
    let mut file = File::open("inputs/day6.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let mut clean_grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        clean_grid.push(row);
    }

    let mut grid = clean_grid.clone();

    // Find the starting point
    let mut starting_position: [usize; 2] = [0, 0];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '^' {
                starting_position[0] = x;
                starting_position[1] = y;
                break;
            }
        }
    }

    run_guard_movement(&mut grid, &starting_position);

    let mut x_count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                //println!("({}, {})", x, y);
                x_count += 1;
            }
        }
        //println!();
    }

    println!("Part 1: {}", x_count);

    // Part 2

    let mut obstacle_count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid = clean_grid.clone();

            grid[y][x] = '#';

            if run_guard_movement(&mut grid, &starting_position) == true {
                obstacle_count += 1;
            }
        }
    }

    println!("Part 2: {}", obstacle_count);
}

fn run_guard_movement(grid: &mut Vec<Vec<char>>, starting_position: &[usize; 2]) -> bool {
    let mut guard_pos: [usize; 2] = starting_position.clone();
    let mut guard_movement = 0; // 0 = UP, 1 = RIGHT, 2 = DOWN, 3 = LEFT
    let mut visited_squares: BTreeMap<[usize; 2], Vec<i32>> = BTreeMap::new();

    let mut next_pos: [usize; 2];
    loop {
        grid[guard_pos[1]][guard_pos[0]] = 'X';

        match guard_movement {
            0 => {
                if guard_pos[1] == 0 {
                    break false;
                }
                next_pos = [guard_pos[0], guard_pos[1] - 1];
            }
            1 => {
                next_pos = [guard_pos[0] + 1, guard_pos[1]];
            }
            2 => {
                next_pos = [guard_pos[0], guard_pos[1] + 1];
            }
            3 => {
                if guard_pos[0] == 0 {
                    break false;
                }
                next_pos = [guard_pos[0] - 1, guard_pos[1]];
            }
            _ => {
                panic!("Invalid movement");
            }
        }

        if next_pos[1] >= grid.len() || next_pos[0] >= grid[next_pos[1]].len() {
            break false;
        }

        if grid[next_pos[1]][next_pos[0]] == '#' {
            // Turn right
            guard_movement = (guard_movement + 1) % 4;

            next_pos = guard_pos;
        }

        guard_pos = next_pos;

        let past = visited_squares.get(&guard_pos);
        if past.is_some() && past.unwrap().contains(&guard_movement) {
            break true;
        }

        if past.is_some() {
            let mut movements = past.unwrap().clone();
            movements.push(guard_movement);
            visited_squares.insert(guard_pos, movements);
        } else {
            visited_squares.insert(guard_pos, vec![guard_movement]);
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x]);
        }
        println!();
    }
}
