use std::{fs, time::Instant};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut reports = Vec::<Vec<i32>>::new();

    let mut i: usize = 0;
    for line in file.lines() {
        reports.push(Vec::<i32>::new());

        for num in line.split_whitespace() {
            let num: i32 = num.parse().expect("Failed to parse number");
            reports[i].push(num);
        }

        i += 1;
    }

    //Part 1
    let now = Instant::now();
    let mut num_safe_reports = 0;

    for report in reports {
        num_safe_reports += if is_valid_report(report) { 1 } else { 0 };
    }

    let time_elapsed = now.elapsed();

    println!("Number of safe reports: {num_safe_reports}");
    println!("Time elapsed: {:?}", time_elapsed);
}

fn is_valid_report(report: Vec<i32>) -> bool {
    let mut total_change: i32 = 0;

    for i in 1..report.len() {
        let change = report[i] - report[i - 1];
        
        if change.abs() < 1 || 3 < change.abs() {
            return false;
        } else if total_change.signum() != 0 && change.signum() != total_change.signum() {
            return false
        }

        total_change += change;
    }

    true
}
