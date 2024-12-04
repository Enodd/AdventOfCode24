use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn parse_number(number: &str) -> i32 {
    let num = number.parse::<i32>();
    match num {
        Ok(number) => {
            return number;
        }
        Err(e) => {
            eprintln!("Error parsing number: {}", e);
            return -1;
        }
    }
}

fn read_file(file_name: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);
    let mut raports: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let mut raport: Vec<i32> = vec![];
        match line {
            Ok(line) => {
                let mut split_line = line.split_whitespace().into_iter();
                while let Some(line) = split_line.next() {
                    let num = parse_number(line);
                    raport.push(num);
                }
                raports.push(raport);
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
    }
    Ok(raports)
}

fn is_safe(raport: &Vec<i32>) -> bool {
    let mut i = 1;
    let mut is_decreasing: bool;
    let mut prev_decreasing: bool;
    let diff = raport[1] - raport[0];
    if diff < 0 {
        prev_decreasing = true;
    } else if diff > 0 {
        prev_decreasing = false;
    } else {
        return false;
    }

    let upper_diff = 3;
    while i < raport.len() {
        let difference = raport[i] - raport[i - 1];
        if difference < 0 {
            is_decreasing = true;
        } else if difference > 0 {
            is_decreasing = false;
        } else {
            return false;
        }

        if difference.abs() > upper_diff {
            return false;
        }

        if is_decreasing == prev_decreasing {
            prev_decreasing = is_decreasing;
        } else {
            return false;
        }
        i += 1;
    }
    true
}

fn is_safe_dampener(raport: &Vec<i32>) -> bool {
    // todo: add additional array
    let mut i = 1;
    let mut is_decreasing: bool = false;
    let mut prev_decreasing: bool = false;

    let diff = raport[1] - raport[0];

    if diff < 0 {
        prev_decreasing = true;
    } else if diff > 0 {
        prev_decreasing = false;
    } else {
        if !check_damp(&raport) {
            return false;
        }
    }

    let upper_diff = 3;
    while i < raport.len() {
        let difference = raport[i] - raport[i - 1];

        if difference < 0 {
            is_decreasing = true;
        } else if difference > 0 {
            is_decreasing = false;
        }

        if difference == 0 || difference.abs() > upper_diff || prev_decreasing != is_decreasing {
            if !check_damp(&raport) {
                return false;
            }
        }

        prev_decreasing = is_decreasing;
        i += 1;
    }
    true
}

fn check_damp(raport: &Vec<i32>) -> bool {
    let mut modified: Vec<i32> = vec![];
    for i in 0..raport.len() {
        modified = raport.clone();
        modified.remove(i);

        if is_safe(&modified) {
            return true;
        }
    }
    let is_new_safe = is_safe(&modified);
    println!(
        "original array: {:?}\nmodified array: {:?}\nis safe: {}\n",
        raport, modified, is_new_safe
    );
    is_new_safe
}

fn main() {
    let file_path = "input.txt";
    let raports = read_file(file_path).unwrap();
    let mut safe_count = 0;
    let mut safe_dump_count = 0;
    for i in raports {
        if is_safe(&i) {
            safe_count += 1;
        }
        if is_safe_dampener(&i) {
            safe_dump_count += 1;
        }
    }
    println!("safe count for {}: {}", file_path, safe_count);
    println!("safe dump count for {}: {}", file_path, safe_dump_count);
}
