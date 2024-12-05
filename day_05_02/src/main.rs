use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").expect("there is a input file");
    let file_reader = BufReader::new(file);

    let mut sum = 0;

    let mut rules: HashMap<u8, Vec<u8>> = HashMap::new();

    let mut parsing_rules = true;

    for line in file_reader.lines() {
        let line = line.expect("line exists");
        let line = line.trim();

        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let slice = line.split('|').collect::<Vec<&str>>();
            let [num1, num2] = [
                slice[0].parse::<u8>().unwrap(),
                slice[1].parse::<u8>().unwrap(),
            ];

            if let Some(vals) = rules.get_mut(&num1) {
                vals.push(num2);
            } else {
                rules.insert(num1, vec![num2]);
            }
        } else {
            let print_order: Vec<u8> = line
                .split(',')
                .map(|el| el.parse::<u8>().unwrap())
                .collect();

            let mut is_incorrect = false;
            let mut corrected_print_order = print_order.clone();

            let mut i: i32 = 0;

            while i < corrected_print_order.len() as i32 {
                let curr = corrected_print_order[i as usize];
                let vals = rules.get(&curr);

                if let Some(vals) = vals {
                    let mut j: i32 = 0;
                    while j < corrected_print_order.len() as i32 {
                        if vals.contains(&corrected_print_order[j as usize]) && j < i {
                            is_incorrect = true;
                            let removed = corrected_print_order.remove(j as usize);
                            corrected_print_order.insert(i as usize, removed);
                            i -= 1;
                            j -= 1;
                        }
                        j += 1;
                    }
                }
                i += 1;
            }

            if is_incorrect {
                let middle = corrected_print_order[(corrected_print_order.len() - 1) / 2];
                sum += middle as usize;
            }
        }
    }

    println!("{}", sum);
}