#![allow(dead_code)]

use std::{fs::File, io::Read};

mod q1;
mod q2;
mod q3;

pub fn read_file(path: &str) -> String {
    let mut input = String::new();
    File::open(path)
        .expect("cannot read file")
        .read_to_string(&mut input)
        .expect("cannot store file contents into string");
    input
}

pub trait ConvertI32 {
    fn to_i32(self) -> i32;
}

impl ConvertI32 for &str {
    fn to_i32(self) -> i32 {
        self.parse().expect("Cannot convert")
    }
}

impl ConvertI32 for String {
    fn to_i32(self) -> i32 {
        self.parse().expect("Cannot convert")
    }
}

fn main() {
    // q1::solve_p2();
    // q2::solve_p2();
    q3::solve_p2();
}
