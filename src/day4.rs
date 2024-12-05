use std::{fs::File, io::Read};

pub fn main() {
    let mut file = File::open("inputs/day4.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let mut count = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                // Check Horizontal
                if x + 3 < grid[y].len()
                    && grid[y][x + 1] == 'M'
                    && grid[y][x + 2] == 'A'
                    && grid[y][x + 3] == 'S'
                {
                    count += 1;
                }
                // Check Vertical
                if y + 3 < grid.len()
                    && grid[y + 1][x] == 'M'
                    && grid[y + 2][x] == 'A'
                    && grid[y + 3][x] == 'S'
                {
                    count += 1;
                }
                // Check Down-Right Diagonal
                if y + 3 < grid.len()
                    && x + 3 < grid[y].len()
                    && grid[y + 1][x + 1] == 'M'
                    && grid[y + 2][x + 2] == 'A'
                    && grid[y + 3][x + 3] == 'S'
                {
                    count += 1;
                }
                // Check Down-Left Diagonal
                if y + 3 < grid.len()
                    && x >= 3
                    && grid[y + 1][x - 1] == 'M'
                    && grid[y + 2][x - 2] == 'A'
                    && grid[y + 3][x - 3] == 'S'
                {
                    count += 1;
                }
                // Check Up-Right Diagonal
                if y >= 3
                    && x + 3 < grid[y].len()
                    && grid[y - 1][x + 1] == 'M'
                    && grid[y - 2][x + 2] == 'A'
                    && grid[y - 3][x + 3] == 'S'
                {
                    count += 1;
                }
                // Check Up-Left Diagonal
                if y >= 3
                    && x >= 3
                    && grid[y - 1][x - 1] == 'M'
                    && grid[y - 2][x - 2] == 'A'
                    && grid[y - 3][x - 3] == 'S'
                {
                    count += 1;
                }
                // Backwards Horizontal
                if x >= 3 && grid[y][x - 1] == 'M' && grid[y][x - 2] == 'A' && grid[y][x - 3] == 'S'
                {
                    count += 1;
                }
                // Backwards Vertical
                if y >= 3 && grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S'
                {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    let mut count2 = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'A' {
                let mut crossCount = 0;

                // Check there's enough space, 3,3 with current index at the centre
                if y >= 1 && y + 1 < grid.len() && x >= 1 && x + 1 < grid[y].len() {
                    // Check for forward crosses
                    // M . M
                    // . A .
                    // S . S
                    if grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S' {
                        crossCount += 1;
                    }

                    if grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S' {
                        crossCount += 1;
                    }

                    // Check for backward crosses
                    // S . S
                    // . A .
                    // M . M
                    if grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M' {
                        crossCount += 1;
                    }

                    if grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M' {
                        crossCount += 1;
                    }

                    if crossCount >= 2 {
                        count2 += 1;
                    }
                }
            }
        }
    }

    println!("{}", count2)
}
