use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let file = File::open("input").expect("there is a input file");
    let file_reader = BufReader::new(file);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;

    for line in file_reader.lines() {
        let line = line.expect("can read line");
        for (_, [num1, num2]) in re.captures_iter(line.as_str()).map(|c| c.extract()) {
            let [num1, num2]: [usize; 2] = [num1.parse().unwrap(), num2.parse().unwrap()];
            sum += num1 * num2;
        }
    }

    println!("{sum}");
}
