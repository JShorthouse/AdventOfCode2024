use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::collections::HashMap;

fn process_stone(stone: i64, memory: &mut HashMap<(i64, usize), i64>, depth: usize, max_depth: usize) -> i64 {
    if (depth > max_depth) {
        return 1;
    }

    // Check if we have processed this stone before, if so just return the number of children from last time
    if let Some(count) = memory.get(&(stone, depth)) {
        return *count;
    }

    let mut stone_count = 0;

    if stone == 0 {
        stone_count += process_stone(1, memory, depth + 1, max_depth);
    } else if (stone.to_string().len() % 2) == 0 {
        let str_val: String = stone.to_string();
        stone_count += process_stone(str_val[..str_val.len()/2].parse().unwrap(), memory, depth + 1, max_depth);
        stone_count += process_stone(str_val[str_val.len()/2..].parse().unwrap(), memory, depth + 1, max_depth);
    } else {
        stone_count += process_stone(stone * 2024, memory, depth + 1, max_depth);
    }

    memory.insert((stone, depth), stone_count);
    return stone_count;
}

fn main() {
    let file = File::open("input/11.txt").unwrap();
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();
    let mut stones: Vec<i64> = line.split_whitespace().map(|v| v.parse().unwrap()).collect();

    println!("{:?}", &stones);

    let mut p1_count = 0;
    let mut p2_count = 0;
    let mut p1_memory = HashMap::new();
    let mut p2_memory = HashMap::new();

    for stone in &stones {
        p1_count += process_stone(*stone, &mut p1_memory, 1, 25);
    }

    for stone in &stones {
        p2_count += process_stone(*stone, &mut p2_memory, 1, 75);
    }

    println!("Part 1: {}", p1_count);
    println!("Part 2: {}", p2_count);
}
