use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::collections::HashMap;

fn main() {
    let file = File::open("input/04.txt").unwrap();
    let reader = BufReader::new(file);

    let mut grid = Vec::<Vec<char>>::new();

    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect();
        grid.push(line);
    }

    let xmas = ['X', 'M','A','S'];
    let mas = ['M','A','S'];
    let mut xmas_count = 0;

    let mut mas_locations = HashMap::<[i32; 2], i32>::new();

    for y in 0..grid.len() as i32 {
        for x in 0..grid[0].len() as i32 {
            let offsets: &[[i32;2]] = &[[1, 0], [-1, 0], [0, 1], [0, -1], [1,1], [1,-1], [-1, 1], [-1, -1]];

            for offset in offsets {
                let end_x = x + offset[0] * (xmas.len()-1) as i32;
                let end_y = y + offset[1] * (xmas.len()-1) as i32;

                if end_x >= 0 && end_x < grid[0].len() as i32 && end_y >= 0 && end_y < grid.len() as i32 {
                    let mut found = true;
                    for i in 0..xmas.len() as i32 {
                        if grid[(y as i32 + offset[1]*i) as usize][(x as i32 + offset[0]*i) as usize] != xmas[i as usize] {
                            found = false;
                        }
                    }
                    if found { 
                        xmas_count += 1; 
                    }
                }
            }

            // Diagonal only, to check for X-MAS
            for offset in offsets.iter().filter(|offset| offset[0] != 0 && offset[1] != 0) {
                let end_x = x + offset[0] * (mas.len()-1) as i32;
                let end_y = y + offset[1] * (mas.len()-1) as i32;

                if end_x >= 0 && end_x < grid[0].len() as i32 && end_y >= 0 && end_y < grid.len() as i32 {
                    let mut found = true;
                    for i in 0..mas.len() as i32 {
                        if grid[(y as i32 + offset[1]*i) as usize][(x as i32 + offset[0]*i) as usize] != mas[i as usize] {
                            found = false;
                        }
                    }
                    if found { 
                        let mid_x = x + offset[0] * 1;
                        let mid_y = y + offset[1] * 1;

                        match mas_locations.get(&[mid_x, mid_y]) {
                            Some(count) => { mas_locations.insert([mid_x, mid_y], count + 1); }
                            None => { mas_locations.insert([mid_x, mid_y], 1); }
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", xmas_count);

    let p2_count = mas_locations.iter().filter(|(_, c)| **c == 2).count();
    println!("Part 2: {}", p2_count);
}
