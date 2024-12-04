use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let file = File::open("input").expect("there is a input file");
    let file_reader = BufReader::new(file);

    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(don\'t\(\))|(do\(\))").unwrap();
    let get_nums_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    let mut is_do = true;

    for line in file_reader.lines() {
        let line = line.expect("can read line");
        for c in re.captures_iter(&line) {
            let curr_match = c.get(0).unwrap().as_str();
            let command = curr_match.split('(').next().unwrap();

            match command {
                "mul" => {
                    if is_do {
                        let captures = get_nums_re.captures(curr_match).unwrap();
                        let (_, [num1, num2]) = captures.extract();

                        let (num1, num2): (usize, usize) =
                            (num1.parse().unwrap(), num2.parse().unwrap());
                        sum += num1 * num2;
                    }
                }
                "don't" => {
                    is_do = false;
                }
                "do" => {
                    is_do = true;
                }
                _ => {}
            }
        }
    }

    println!("{sum}");
}
