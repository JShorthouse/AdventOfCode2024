use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};
use std::collections::HashMap;

fn main() {
    let file = File::open("input/01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut list1 = Vec::<i32>::new();
    let mut list2 = Vec::<i32>::new();

    let mut list2_counts = HashMap::<i32, i32>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split("   ");

        let num1 = split.next().unwrap().parse::<i32>().unwrap();
        let num2 = split.next().unwrap().parse::<i32>().unwrap();
         
        list1.push(num1);
        list2.push(num2);

        match list2_counts.get(&num2) {
            Some(count) => list2_counts.insert(num2, count + 1),
            None => list2_counts.insert(num2, 1),
        };
    }

    list1.sort();
    list2.sort();

    let mut diff_total = 0;
    for i in 0..list1.len() {
        diff_total += (list1[i] - list2[i]).abs()
    }

    println!("Part 1: {}", diff_total);

    let mut p2_total = 0;

    for num in list1 {
        match list2_counts.get(&num) {
            Some(count) => p2_total += num * count,
            None => {},
        };
    }

    println!("Part 2: {}", p2_total);
}
