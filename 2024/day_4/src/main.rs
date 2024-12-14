use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut matrix = Vec::<Vec<char>>::new();

    let mut i = 0;
    for line in file.lines() {
        matrix.push(Vec::<char>::new());
        let line = line.trim();

        for char in line.chars() {
            matrix[i].push(char);
        }

        i += 1;
    }

    // Part 1

    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut xmas_count = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != 'X' {
                continue;
            }

            for (y, x) in offsets {
                let i = i as i32;
                let j = j as i32;

                let offset3_y = i - y * 3;
                let offset3_x = j - x * 3;

                if offset3_y.is_negative()
                    || offset3_x.is_negative()
                    || offset3_y >= matrix.len() as i32
                    || offset3_x >= matrix[0].len() as i32
                {
                    continue;
                }

                let offset1_y = i - y;
                let offset1_x = j - x;

                let offset2_y = i - y * 2;
                let offset2_x = j - x * 2;

                if matrix[offset1_y as usize][offset1_x as usize] != 'M' {
                    continue;
                } else if matrix[offset2_y as usize][offset2_x as usize] != 'A' {
                    continue;
                } else if matrix[offset3_y as usize][offset3_x as usize] != 'S' {
                    continue;
                } else {
                    xmas_count += 1;
                }
            }
        }
    }

    // Part 2

    let mut x_mas_count = 0;

    for i in 1..(matrix.len() - 1) {
        for j in 1..(matrix[0].len() - 1) {
            if matrix[i][j] != 'A' {
                continue;
            }

            let top_left = matrix[i - 1][j - 1];
            let bottom_right = matrix[i + 1][j + 1];

            let bottom_left = matrix[i + 1][j - 1];
            let top_right = matrix[i - 1][j + 1];

            if !((top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M'))
            {
                continue;
            } else if !((bottom_left == 'M' && top_right == 'S')
                || (bottom_left == 'S' && top_right == 'M'))
            {
                continue;
            } else {
                x_mas_count += 1;
            }
        }
    }

    println!("Xmas Count: {xmas_count}");
    println!("X-Mas Count: {x_mas_count}");
}
