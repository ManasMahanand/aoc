use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input").expect("there is a input file");
    let file_reader = BufReader::new(file);
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut sum = 0;

    for line in file_reader.lines() {
        let line = line.expect("line exists");
        matrix.push(line.chars().collect());
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let cur_char = matrix[i][j];

            if cur_char != 'X' {
                continue;
            }

            if matrix[i].len() - j >= 4 {
                let horizontal: String = matrix[i][j..(j + 4)].iter().collect();
                if horizontal == "XMAS" {
                    sum += 1;
                }
            }

            if j >= 3 {
                let mut slice = matrix[i][(j - 3)..=j].to_vec();
                slice.reverse();
                let horizontal_reverse: String = slice.iter().collect();
                if horizontal_reverse == "XMAS" {
                    sum += 1;
                }
            }

            if matrix.len() - i >= 4 {
                let vertical: String = matrix[i..(i + 4)].iter().map(|inner| inner[j]).collect();
                if vertical == "XMAS" {
                    sum += 1;
                }
            }

            if i >= 3 {
                let mut slice = matrix[(i - 3)..=i].to_vec();
                slice.reverse();
                let vertical_reverse: String = slice.iter().map(|inner| inner[j]).collect();
                if vertical_reverse == "XMAS" {
                    sum += 1;
                }
            }

            if matrix.len() - i >= 4 && matrix[i].len() - j >= 4 {
                let mut diagonal_bottom_right = String::new();
                let slice = &matrix[i..(i + 4)];

                for k in 0..4 {
                    diagonal_bottom_right.push(slice[k][j + k]);
                }

                if diagonal_bottom_right == "XMAS" {
                    sum += 1;
                }
            }

            if matrix.len() - i >= 4 && j >= 3 {
                let mut diagonal_bottom_left = String::new();
                let slice = &matrix[i..(i + 4)];

                for k in 0..4 {
                    diagonal_bottom_left.push(slice[k][j - k]);
                }

                if diagonal_bottom_left == "XMAS" {
                    sum += 1;
                }
            }

            if i >= 3 && j >= 3 {
                let mut diagonal_top_left = String::new();
                let mut slice = matrix[(i - 3)..=i].to_vec();
                slice.reverse();

                for k in 0..4 {
                    diagonal_top_left.push(slice[k][j - k]);
                }

                if diagonal_top_left == "XMAS" {
                    sum += 1;
                }
            }

            if i >= 3 && matrix[i].len() - j >= 4 {
                let mut diagonal_top_right = String::new();
                let mut slice = matrix[(i - 3)..=i].to_vec();
                slice.reverse();

                for k in 0..4 {
                    diagonal_top_right.push(slice[k][j + k]);
                }

                if diagonal_top_right == "XMAS" {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}
