use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

struct InputValues {
    goal: i64,
    numbers: Vec<i64>,
}

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

fn main() {
    let file = File::open("input/07.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inputs = Vec::<InputValues>::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(':');
        let goal: i64 = split.next().unwrap().parse().unwrap();
        let numbers: Vec<i64> = split.next().unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect();

        inputs.push( InputValues{goal, numbers} );
    }

    let mut p1_sum = 0;

    for input in inputs {
        // Build combination of operators
        for i in 0..2_u32.pow(input.numbers.len() as u32 - 1) {
            let mut operators = Vec::<Operator>::new();

            // Build combinations of operators using binary digits and bitmask
            let mut mask = 1;
            for _ in 0..(input.numbers.len()-1) {
                if i & mask > 0 {
                    operators.push(Operator::Add);
                } else {
                    operators.push(Operator::Multiply);
                }
                mask <<= 1;
            }

            let mut total = input.numbers[0];
            for digit_num in 1..input.numbers.len() {
                match operators[digit_num-1] {
                    Operator::Add => total += input.numbers[digit_num],
                    Operator::Multiply => total *= input.numbers[digit_num],
                }
            }

            if total == input.goal {
                p1_sum += input.goal;
                break;
            }
        }
    }

    println!("Part 1: {}", p1_sum);
}
