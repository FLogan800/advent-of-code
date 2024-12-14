use regex::Regex;
use std::{fs, time::Instant};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut skip = false;

    let start = Instant::now();

    for mat in re.captures_iter(&file) {
        if &mat[0] == "do()" {
            skip = false;
        } else if &mat[0] == "don't()" {
            skip = true;
        } else if !skip {
            let num1: i32 = mat[1].parse().unwrap();
            let num2: i32 = mat[2].parse().unwrap();

            sum += num1 * num2;
        }
    }
    let time_elapsed = start.elapsed();

    println!("Sum: {sum}");
    println!("Time elapsed: {:?}", time_elapsed);
}
