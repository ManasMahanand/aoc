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

    for i in 1..matrix.len() {
        for j in 1..matrix[i].len() {
            let cur_char = matrix[i][j];

            if cur_char != 'A' {
                continue;
            }

            let mut found_diagonal_left = false;
            let mut found_diagonal_right = false;

            if matrix.len() - i >= 2 && matrix[i].len() - j >= 2 {
                let mut diagonal_left = String::new();
                let mut diagonal_right = String::new();

                let slice = &matrix[(i - 1)..=(i + 1)];

                for k in 0..3 {
                    diagonal_left.push(slice[k][j - 1 + k]);
                }

                if diagonal_left == "MAS" || diagonal_left == "SAM" {
                    found_diagonal_left = true;
                }

                let mut slice = slice.to_vec();
                slice.reverse();

                for k in 0..3 {
                    diagonal_right.push(slice[k][j - 1 + k]);
                }

                if diagonal_right == "MAS" || diagonal_right == "SAM" {
                    found_diagonal_right = true;
                }
            }

            if found_diagonal_left && found_diagonal_right {
                sum += 1;
            }
        }
    }

    println!("{}", sum);
}
