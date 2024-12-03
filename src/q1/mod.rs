use std::{collections::HashMap, fs::File, io::Read};

pub fn solve_p1() {
    let mut input = String::new();
    File::open("./src/q1/final.txt")
        .expect("cannot read file")
        .read_to_string(&mut input)
        .expect("cannot store file contents into string");

    println!("{input}");

    let (mut left_col, mut right_col): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("   ").expect("Cannot split"))
        .map(|(left_str, right_str)| {
            (
                left_str.parse::<i32>().expect("Cannot convert to number"),
                right_str.parse::<i32>().expect("Cannot convert to number"),
            )
        })
        .unzip();

    left_col.sort();
    right_col.sort();

    let sum: i32 = left_col
        .into_iter()
        .zip(right_col.into_iter())
        .map(|(left, right)| {
            let result = left - right;
            if result < 0 {
                result * -1
            } else {
                result
            }
        })
        .sum();

    println!("{sum}")
}

pub fn solve_p2() {
    let mut input = String::new();
    File::open("./src/q1/final.txt")
        .expect("cannot read file")
        .read_to_string(&mut input)
        .expect("cannot store file contents into string");

    println!("{input}");

    let (left_col, right_col): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split_once("   ").expect("Cannot split"))
        .map(|(left_str, right_str)| {
            (
                left_str.parse::<i32>().expect("Cannot convert to number"),
                right_str.parse::<i32>().expect("Cannot convert to number"),
            )
        })
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
