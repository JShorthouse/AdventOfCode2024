use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use regex::Regex;

fn main() {
    let file = File::open("input/03.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    let mut p2_total = 0;

    let mut p2_enabled = true;

    for line in reader.lines() {
        let line = line.unwrap();

        let re = Regex::new(r"mul\((\d+),(\d+)\)|don't|do").unwrap();

        for cap in re.captures_iter(&line) {
            if cap[0].starts_with("mul") {
                let first: i32 = cap[1].parse().unwrap();
                let second: i32 = cap[2].parse().unwrap();

                total += first * second;

                if p2_enabled {
                    p2_total += first * second;
                }
            } else if cap[0].starts_with("don't") {
                p2_enabled = false;
            } else if cap[0].starts_with("do") {
                p2_enabled = true;
            }
        }
    }

    println!("Part 1: {}", total);
    println!("Part 2: {}", p2_total);
}
