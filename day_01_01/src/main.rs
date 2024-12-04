use std::{
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

        let nums = line.split_whitespace().collect::<Vec<&str>>();

        numbers_1.push(nums[0].parse::<usize>().unwrap());
        numbers_2.push(nums[1].parse::<usize>().unwrap());
    }

    numbers_1.sort();
    numbers_2.sort();

    let mut sum = 0;

    for (i, curr) in numbers_1.iter().enumerate() {
        let num_2 = numbers_2[i];
        sum += curr.abs_diff(num_2);
    }

    println!("{}", sum);
}
