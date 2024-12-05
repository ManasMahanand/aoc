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

    'line_iterator: for line in file_reader.lines() {
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

            for i in 0..print_order.len() {
                let curr = print_order[i];
                let vals = rules.get(&curr);

                if let Some(vals) = vals {
                    for j in 0..print_order.len() {
                        if vals.contains(&print_order[j]) && j < i {
                            continue 'line_iterator;
                        }
                    }
                }
            }

            let middle = print_order[(print_order.len() - 1) / 2];
            sum += middle as usize;
        }
    }

    println!("{}", sum);
}
