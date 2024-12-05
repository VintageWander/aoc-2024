use crate::{read_file, ConvertI32};

pub fn solve_p1() {
    let regex = regex::Regex::new(r"mul\(([0-9]+),([0-9]+)\)").expect("Invalid regex");

    let result: i32 = regex
        .captures_iter(&read_file("./src/q3/final.txt"))
        .map(|capture| {
            capture.get(1).unwrap().as_str().to_i32() * capture.get(2).unwrap().as_str().to_i32()
        })
        .sum();
    println!("{result}")
}

pub fn solve_p2() {
    let regex =
        regex::Regex::new(r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))").expect("Invalid regex");

    let mut on = true;

    let result: i32 = regex
        .captures_iter(&read_file("./src/q3/final.txt"))
        .filter(|capture| {
            if capture.get(0).unwrap().as_str() == "do()" {
                on = true;
                return false;
            } else if capture.get(0).unwrap().as_str() == "don't()" {
                on = false;
            }
            on
        })
        .map(|capture| {
            capture.get(2).unwrap().as_str().to_i32() * capture.get(3).unwrap().as_str().to_i32()
        })
        .sum();
    println!("{result}")
}
