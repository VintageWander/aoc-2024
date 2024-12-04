use std::collections::HashMap;

use crate::{read_file, ConvertI32};

pub fn solve_p1() {
    let (mut left_col, mut right_col): (Vec<i32>, Vec<i32>) = read_file("./src/q1/final.txt")
        .lines()
        .map(|line| line.split_once("   ").expect("Cannot split"))
        .map(|(left_str, right_str)| (left_str.to_i32(), right_str.to_i32()))
        .unzip();

    left_col.sort();
    right_col.sort();

    let sum: u32 = left_col
        .into_iter()
        .zip(right_col)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    println!("{sum}")
}

pub fn solve_p2() {
    let (left_col, right_col): (Vec<i32>, Vec<i32>) = read_file("./src/q1/final.txt")
        .lines()
        .map(|line| line.split_once("   ").expect("Cannot split"))
        .map(|(left_str, right_str)| (left_str.to_i32(), right_str.to_i32()))
        .unzip();

    // Hashmap to store number - occurence
    let mut right_hashmap = HashMap::<i32, i32>::new();

    for num in right_col {
        if let Some(stored_value) = right_hashmap.get_mut(&num) {
            *stored_value += num;
        } else {
            right_hashmap.insert(num, num);
        }
    }

    let result: i32 = left_col
        .into_iter()
        .map(|num| right_hashmap.get(&num).unwrap_or(&0))
        .sum();

    println!("{result}")
}
