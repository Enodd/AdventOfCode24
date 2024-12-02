use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn merge_sort(arr: &mut [i32]) -> Vec<i32> {
    let len = arr.len();
    if len <= 1 {
        return vec![];
    }

    let mid = len / 2;
    let mut left = arr[0..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(arr, &left, &right)
}

fn merge(arr: &mut [i32], left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
    arr.to_vec()
}

pub struct IdLists {
    pub list_one: Vec<i32>,
    pub list_two: Vec<i32>,
}

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

fn read_file() -> io::Result<IdLists> {
    let file_path = "input.txt";

    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    let file = File::open(file_path)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut split_line = line.split_whitespace().into_iter();
                let word1 = parse_number(split_line.next().unwrap());
                let word2 = parse_number(split_line.next().unwrap());
                list_one.push(word1);
                list_two.push(word2);
            }
            Err(e) => {
                eprintln!("Error reading the file: {}", e);
            }
        }
    }

    let sorted_list_one = merge_sort(&mut list_one);
    let sorted_list_two = merge_sort(&mut list_two);
    let lists = IdLists {
        list_one: sorted_list_one,
        list_two: sorted_list_two,
    };

    Ok(lists)
}

fn calculate_difference(list_one: &mut [i32], list_two: &mut [i32]) -> i32 {
    let mut i = 0;
    let mut distances: Vec<i32> = vec![];

    while i < list_one.len() && i < list_two.len() {
        let difference = list_one[i] - list_two[i];
        distances.push(difference.abs());
        i += 1;
    }
    let mut total: i32 = 0;

    for i in distances {
        total += i;
    }
    total
}

fn calculate_similarity(list_one: &mut [i32], list_two: &mut [i32]) -> i32 {
    let mut i = 0;
    let mut similarities: Vec<i32> = vec![];
    let mut temp_arr: Vec<i32> = vec![];
    list_two.clone_into(&mut temp_arr);

    while i < list_one.len() {
        let mut similarity = 0;
        let value = list_one[i];
        for j in temp_arr.clone() {
            if j == value {
                similarity += 1;
            }
        }
        similarities.push(similarity * value);
        i += 1;
    }

    let mut total: i32 = 0;
    for i in similarities {
        total += i;
    }
    total
}

fn main() {
    let mut lists = read_file().unwrap();
    let total = calculate_difference(&mut lists.list_one, &mut lists.list_two);
    let sim = calculate_similarity(&mut lists.list_one, &mut lists.list_two);

    println!("calculated difference: {}", total);
    println!("calculated similarities: {}", sim);
}
