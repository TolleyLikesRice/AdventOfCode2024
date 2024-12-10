use core::fmt;
use std::{fs::File, io::Read};

pub fn main() {
    let mut file = File::open("inputs/day9.txt").expect("Unable to open file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read file");

    let mut disk: Vec<String> = Vec::new();
    let mut disk_part2: Vec<Chunk> = Vec::new();

    let mut id = 0;

    for (i, char) in input.chars().enumerate() {
        if i % 2 == 0 {
            // Data
            let length = char.to_digit(10).unwrap();
            for _ in 0..length {
                disk.push(id.to_string());
            }
            disk_part2.push(Chunk {
                id: Some(id as usize),
                length: length as usize,
            });
            id += 1;
        } else {
            // Free Space
            let length = char.to_digit(10).unwrap();
            for _ in 0..length {
                disk.push(String::from("."));
            }
            disk_part2.push(Chunk {
                id: None,
                length: length as usize,
            });
        }
    }

    let mut new_disk: Vec<String> = Vec::new();
    for i in 0..disk.len() {
        if i >= disk.len() {
            break;
        }

        let sector = &disk[i];

        if sector == "." {
            let mut new_data = disk.pop().unwrap();
            while new_data == "." {
                new_data = disk.pop().unwrap();
            }
            new_disk.push(new_data);
        } else {
            new_disk.push(sector.clone());
        }
    }

    let mut checksum = 0;

    for (i, sector) in new_disk.iter().enumerate() {
        let sector_value = sector.parse::<usize>().unwrap();
        checksum += sector_value * i;
    }

    println!("Part 1: {}", checksum);

    // Part 2

    let highest_id = disk_part2
        .iter()
        .max_by_key(|x| x.id.unwrap_or_default())
        .unwrap()
        .id
        .unwrap();

    // Iterate through backwards
    for id in (0..highest_id + 1).rev() {
        let data_chunk = disk_part2
            .iter()
            .find(|x| x.id == Some(id))
            .unwrap()
            .clone();
        let data_chunk_index = disk_part2.iter().position(|x| x.id == Some(id)).unwrap();

        let mut empty_index: Option<usize> = None;
        let mut delete_empty = false;
        for (i, chunk) in disk_part2.iter_mut().enumerate() {
            // Empty chunk with enough space
            if chunk.id.is_none() && chunk.length >= data_chunk.length {
                if i > data_chunk_index {
                    break;
                }
                chunk.length -= data_chunk.length;

                empty_index = Some(i);
                if chunk.length == 0 {
                    delete_empty = true;
                }
                break;
            }
        }

        if empty_index.is_some() {
            disk_part2[data_chunk_index].id = None;
            disk_part2.insert(empty_index.unwrap(), data_chunk);
            if delete_empty {
                disk_part2.remove(empty_index.unwrap() + 1);
            }
        }
    }

    // Calculate checksum for part 2
    let mut checksum = 0;
    let mut i = 0;
    for chunk in disk_part2 {
        if chunk.id.is_some() {
            for _ in 0..chunk.length {
                checksum += chunk.id.unwrap() * i;
                i += 1;
            }
        } else {
            i += chunk.length;
        }
    }

    println!("Part 2: {}", checksum);
}

#[derive(Clone)]
struct Chunk {
    pub id: Option<usize>,
    pub length: usize,
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.id.is_none() {
            let mut out: Vec<String> = Vec::new();
            for _ in 0..self.length {
                out.push(".".to_string());
            }
            write!(f, "{}", out.join(""))
        } else {
            let mut out: Vec<String> = Vec::new();
            for _ in 0..self.length {
                out.push(self.id.unwrap().to_string());
            }
            write!(f, "{}", out.join(""))
        }
    }
}
