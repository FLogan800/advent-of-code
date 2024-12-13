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

    let now = Instant::now();
    let mut num_safe_reports = 0;

    for report in reports {
        let mut is_safe = is_valid_report(report.clone(), 99);
        
        if !is_safe {
            for i in 0..report.len() {
                if is_valid_report(report.clone(), i) {
                    is_safe = true;
                    break;
                }
            }
        }

        num_safe_reports += if is_safe { 1 } else { 0 };
    }

    let time_elapsed = now.elapsed();

    println!("Number of safe reports: {num_safe_reports}");
    println!("Time elapsed: {:?}", time_elapsed);
}

fn is_valid_report(mut report: Vec<i32>, remove: usize) -> bool {
    if remove <= report.len() {
        report.remove(remove as usize);
    }

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
