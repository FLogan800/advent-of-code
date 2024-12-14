use std::{fs, time::Instant};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let split = file.split("\n\n").collect::<Vec<&str>>();

    let mut rules = Vec::<(i32, i32)>::new();
    let mut updates = Vec::<Vec<i32>>::new();

    for rule in split[0].split_whitespace() {
        let start_end = rule.split("|").collect::<Vec<&str>>();
        let first_page: i32 = start_end[0].parse().expect("Failed to parse starting page");
        let second_page: i32 = start_end[1]
            .parse()
            .expect("Failed to parse the second page");

        rules.push((first_page, second_page));
    }

    for (i, update) in split[1].split_whitespace().enumerate() {
        updates.push(Vec::<i32>::new());

        for page in update.split(",") {
            let page = page.parse().unwrap();
            updates[i].push(page);
        }
    }

    let now = Instant::now();

    let mut valid_sum = 0;
    let mut invalid_sum = 0;

    for update in &mut updates {
        let mut properly_ordered = true;

        while !is_valid(update, &rules) {
            properly_ordered = false;

            for rule in &rules {
                let first_page_index = match update.iter().position(|&r| r == rule.0) {
                    Some(index) => index,
                    None => continue,
                };

                let second_page_index = match update.iter().position(|&r| r == rule.1) {
                    Some(index) => index,
                    None => continue,
                };

                if !validate_rule(update, rule) {
                    update.swap(first_page_index, second_page_index);
                }
            }
        }

        if properly_ordered {
            valid_sum += update[update.len() / 2];
        } else {
            invalid_sum += update[update.len() / 2];
        }
    }
    
    let time_elapsed = now.elapsed();

    println!("Valid Sum: {valid_sum}");
    println!("Invalid Sum: {invalid_sum}");
    println!("Time Elapsed: {:?}", time_elapsed);
}

fn is_valid(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let mut is_valid = true;

    for rule in rules {
        if !validate_rule(update, rule) {
            is_valid = false;
        }
    }

    return is_valid;
}

fn validate_rule(update: &Vec<i32>, rule: &(i32, i32)) -> bool {
    let first_page_index = match update.iter().position(|&r| r == rule.0) {
        Some(index) => index,
        None => return true,
    };

    let second_page_index = match update.iter().position(|&r| r == rule.1) {
        Some(index) => index,
        None => return true,
    };

    if first_page_index < second_page_index {
        return true;
    } else {
        return false;
    }
}
