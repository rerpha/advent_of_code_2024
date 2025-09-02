use std::fs;

fn day1() {
    let contents =
        fs::read_to_string("inputs/1.txt").expect("Something went wrong reading the file");

    // PART 1
    let blah: Vec<(usize, usize)> = contents
        .lines()
        .map(|s| s.split_once("   ").expect("something bad happened"))
        .map(|t| {
            (
                t.0.parse().expect("Error parsing A"),
                t.1.parse().expect("Error parsing B"),
            )
        })
        .collect();

    let mut left_vec: Vec<usize> = blah.iter().map(|t| t.0).collect();
    left_vec.sort();

    let mut right_vec: Vec<usize> = blah.iter().map(|t| t.1).collect();
    right_vec.sort();
    let result: usize = left_vec
        .iter()
        .zip(right_vec.iter())
        .map(|t| t.0.abs_diff(*t.1))
        .sum();
    println!("{result}");

    // PART 2

    let res: usize = left_vec
        .iter()
        .map(|x| right_vec.iter().filter(|&n| *n == *x).count() * x)
        .sum();

    println!("{res}");
}

fn main() {
    day1();
}
