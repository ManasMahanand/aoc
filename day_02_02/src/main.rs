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

        let is_safe_ascending = get_is_safe(&levels, None);
        levels.reverse();
        let is_safe_descending = get_is_safe(&levels, None);

        if is_safe_descending || is_safe_ascending {
            sum += 1;
        }
    }

    println!("{sum}");
}

fn get_is_safe(levels: &[usize], mut el_to_remove: Option<usize>) -> bool {
    let mut is_safe = true;

    for (i, curr) in levels.iter().enumerate() {
        if !(i != 0 && el_to_remove != Some(levels[i - 1])) {
            continue;
        }

        let prev = levels[i - 1];
        let curr = *curr;
        let next = if i < levels.len() - 1 {
            levels[i + 1]
        } else {
            0
        };

        let mut curr_not_safe = false;

        if (prev >= curr) || (curr - prev) > 3 {
            curr_not_safe = true;
        }

        if curr_not_safe && i == levels.len() - 1 && el_to_remove.is_none() {
            break;
        }

        if curr_not_safe && i < levels.len() - 1 && ((prev < next) && (next - prev) <= 3) {
            if el_to_remove.is_some() {
                is_safe = false;
                break;
            }
            el_to_remove = Some(curr);
        } else if curr_not_safe {
            is_safe = false;
            break;
        }
    }

    if !is_safe && el_to_remove.is_none() && (levels[0] >= levels[1] || (levels[1] - levels[0]) > 3)
    {
        let mut levels: Vec<usize> = levels.to_vec();
        levels.remove(0);
        get_is_safe(&levels, Some(0))
    } else {
        is_safe
    }
}
