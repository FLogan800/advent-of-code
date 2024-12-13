use std::fs;
use std::time::Instant;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Failed to read file");

    let (mut arr1, mut arr2) = read_file(file);

    arr1.sort();
    arr2.sort();

    let start = Instant::now();
    // Part 1
    let mut total_distance = 0;

    for i in 0..arr1.len() {
        total_distance += (arr1[i] - arr2[i]).abs();
    }
    
    // Part 2
    let mut similarity_score = 0;
    
    for i in 0..arr1.len() {
        let mut appearances = 0;
        
        for j in 0..arr2.len() {
            if arr1[i] == arr2[j] {
                appearances += 1;
            }
        }
        
        similarity_score += arr1[i] * appearances;
    }
    let end = start.elapsed();
    
    println!("Total Distance: {total_distance}");
    println!("Similarity Score: {similarity_score}");
    println!("Time Elapsed: {:?}", end);
}

fn read_file(file: String) -> (Vec<i32>, Vec<i32>) {
    let mut arr1 = Vec::<i32>::new();
    let mut arr2 = Vec::<i32>::new();

    for num in file.split_whitespace() {
        let num: i32 = num.parse().expect("Failed to parse number");

        if arr1.len() == arr2.len() {
            arr1.push(num);
        } else {
            arr2.push(num);
        }
    }

    (arr1, arr2)
}
