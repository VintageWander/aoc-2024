#![allow(dead_code)]

use std::{fs::File, io::Read};

mod q1;
mod q2;

pub fn read_file(path: &str) -> String {
    let mut input = String::new();
    File::open(path)
        .expect("cannot read file")
        .read_to_string(&mut input)
        .expect("cannot store file contents into string");
    input
}

fn main() {
    // q1::solve_p2();
    q2::solve_p1();
}
