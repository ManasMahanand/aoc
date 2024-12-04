use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").expect("there is a input file");
    let file_reader = BufReader::new(file);

    let mut numbers_1: Vec<usize> = Vec::new();
    let mut numbers_2: Vec<usize> = Vec::new();

    for line in file_reader.lines() {
        let line = line.expect("line exists");

        let nums: Vec<&str> = line.split_whitespace().collect();

        numbers_1.push(nums[0].parse::<usize>().unwrap());
        numbers_2.push(nums[1].parse::<usize>().unwrap());
    }

    numbers_2.sort();

    let mut sum = 0;

    let mut frequency_map: HashMap<usize, usize> = HashMap::new();

    for curr in numbers_1.iter() {
        if let Some(memo) = frequency_map.get(curr) {
            sum += memo;
            continue;
        };

        let repeated = numbers_2
            .iter()
            .filter(|n| **n == *curr)
            .collect::<Vec<&usize>>()
            .len();

        let memo = curr * repeated;
        frequency_map.insert(*curr, memo);

        sum += memo;
    }
    println!("{}", sum);
}
