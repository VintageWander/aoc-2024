use crate::read_file;

pub fn solve_p1() {
    let mut safe_counter = 0;
    let input = read_file("./src/q2/final.txt");
    input.lines().for_each(|line| {
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().expect("Incorrect data"))
            .collect::<Vec<_>>();
        let mut is_up = false;
        for i in 0..nums.len() {
            if let (Some(first), Some(second)) = (nums.get(i), nums.get(i + 1)) {
                if first == second {
                    break;
                }
                // First run, setting the direction
                if i == 0 {
                    is_up = first < second;
                    // Rest of the run, ensure to follow the direction
                }
                if (is_up && (first > second || second - first > 3))
                    || (!is_up && (first < second || first - second > 3))
                {
                    break;
                }
            } else {
                safe_counter += 1;
                break;
            }
        }
    });
    println!("{safe_counter}");
}
