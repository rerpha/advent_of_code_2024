use std::fs;

fn day1() {
    let contents =
        fs::read_to_string("inputs/1.txt").expect("Something went wrong reading the file");

    let blah: Vec<(u32, u32)> = contents
        .lines()
        .map(|s| s.split_once("   ").expect("something bad happened"))
        .map(|t| {
            (
                t.0.parse().expect("Error parsing A"),
                t.1.parse().expect("Error parsing B"),
            )
        })
        .collect();

    let mut left_vec: Vec<u32> = blah.iter().map(|t| t.0).collect();
    left_vec.sort();

    let mut right_vec: Vec<u32> = blah.iter().map(|t| t.1).collect();
    right_vec.sort();
    let result: u32 = left_vec
        .iter()
        .zip(right_vec.iter())
        .map(|t| t.0.abs_diff(*t.1))
        .sum();
    println!("{result}");
}

fn main() {
    day1();
}
