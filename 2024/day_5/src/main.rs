use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let split = file.split("\n\n").collect::<Vec<&str>>();

    let mut rules = Vec::<(i32, i32)>::new();
    let mut updates = Vec::<Vec<i32>>::new();

    for rule in split[0].split_whitespace() {
        let start_end = rule.split("|").collect::<Vec<&str>>();
        let first_page: i32 = start_end[0].parse().expect("Failed to parse starting page");
        let second_page: i32 = start_end[1].parse().expect("Failed to parse the second page");

        rules.push((first_page, second_page));
    }

    for (i, update) in split[1].split_whitespace().enumerate() {
        updates.push(Vec::<i32>::new());

        for page in update.split(",") {
            let page = page.parse().unwrap();
            updates[i].push(page);
        }
    }

    let mut sum = 0;

    for update in &updates {
        let mut is_valid = true;
        for rule in &rules {
            if !validate_update(update, rule) {
                is_valid = false;
            }
        }

        if is_valid {
            let middle_index = update.len() / 2;
            sum += update[middle_index];
        }
    }

    println!("Sum: {sum}");
}

fn validate_update(update: &Vec<i32>, rule: &(i32, i32)) -> bool{
    let first_page_index = match update.iter().position(|&r| r == rule.0) {
        Some(index) => index,
        None => return true,
    };

    let second_page_index = match update.iter().position(|&r| r == rule.1) {
        Some(index) => index,
        None => return true,
    };

    if first_page_index < second_page_index {
        return true
    } else {
        return false
    }
}
