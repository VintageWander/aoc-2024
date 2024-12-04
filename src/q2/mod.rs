use itertools::Itertools;

use crate::read_file;

pub fn solve_p1() {
    let result = read_file("./src/q2/final.txt")
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<isize>().expect("Incorrect data"))
                .tuple_windows()
                .try_fold(0, |ord, (a, b)| {
                    if ord >= 0 && (1..=3).contains(&(b - a)) {
                        Ok(1)
                    } else if ord <= 0 && (1..=3).contains(&(a - b)) {
                        Ok(-1)
                    } else {
                        Err(())
                    }
                })
                .is_ok()
        })
        .count();

    println!("{result}");
}

pub fn solve_p2() {
    let result = read_file("./src/q2/final.txt")
        .lines()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|num_str| num_str.parse::<isize>().expect("Incorrect data"))
                .collect::<Vec<_>>();

            (0..nums.len()).any(|i| {
                nums[0..i]
                    .iter()
                    .chain(&nums[i + 1..])
                    .tuple_windows()
                    .try_fold(0, |ord, (first, second)| {
                        if ord >= 0 && (1..=3).contains(&(second - first)) {
                            Ok(1)
                        } else if ord <= 0 && (1..=3).contains(&(first - second)) {
                            Ok(-1)
                        } else {
                            Err(())
                        }
                    })
                    .is_ok()
            })
        })
        .count();

    println!("{result}");
}
