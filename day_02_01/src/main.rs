use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").expect("there is a input file");
    let file_reader = BufReader::new(file);

    let mut sum = 0;
    for line in file_reader.lines() {
        let line = line.expect("line exists");

        let mut levels: Vec<usize> = line
            .split_whitespace()
            .map(|el| el.parse::<usize>().unwrap())
            .collect();

        let is_safe_ascending = get_is_safe(&levels);
        levels.reverse();
        let is_safe_descending = get_is_safe(&levels);

        if is_safe_descending || is_safe_ascending {
            sum += 1;
        }
    }

    println!("{sum}");
}

fn get_is_safe(levels: &[usize]) -> bool {
    if levels.is_sorted() {
        let mut is_safe = true;

        for (i, curr) in levels.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let prev = levels[i - 1];

            if (prev == *curr) || (*curr - prev) > 3 {
                is_safe = false;
                break;
            }
        }

        is_safe
    } else {
        false
    }
}
