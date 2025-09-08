use itertools::Itertools;
use regex::Regex;
use std::fs;

fn day1() {
    let contents =
        fs::read_to_string("inputs/1.txt").expect("Something went wrong reading the file");

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

    // PART 1
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

fn level_is_safe(level: &Vec<usize>) -> bool {
    let adjacent_out_of_range = level
        .windows(2)
        .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));

    (level.is_sorted_by(|a, b| a <= b) || level.is_sorted_by(|a, b| a >= b))
        && adjacent_out_of_range
}

fn level_is_sort_of_safe(level: &Vec<usize>) -> bool {
    level
        .iter()
        .cloned()
        .combinations(level.len() - 1)
        .map(|x| x[0..x.len()].to_owned())
        .map(|x| level_is_safe(&x))
        .any(|x| x == true)
}

fn day2() {
    // PART 1
    let contents =
        fs::read_to_string("inputs/2.txt").expect("Something went wrong reading the file");

    let parsed_levels: Vec<Vec<usize>> = contents
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|z| z.parse::<usize>().expect("help"))
                .collect()
        })
        .collect();

    let unsafe_levels: usize = parsed_levels
        .iter()
        .filter(|level| level_is_safe(level))
        .count();

    println!("{:?}", unsafe_levels);

    // PART 2
    let sort_of_unsafe_levels: usize = parsed_levels
        .iter()
        .filter(|level| level_is_safe(level) || level_is_sort_of_safe(level))
        .count();
    println!("{:?}", sort_of_unsafe_levels);
}

fn day3() {
    // part 1
    let contents =
        fs::read_to_string("inputs/3.txt").expect("Something went wrong reading the file");

    let re = Regex::new(r"mul\((\d{0,3}),(\d{0,3})\)").unwrap();

    let mut results = vec![];

    for (_, [firstnum, secondnum]) in re.captures_iter(&contents).map(|c| c.extract()) {
        results.push(firstnum.parse::<usize>().unwrap() * secondnum.parse::<usize>().unwrap());
    }

    let total: usize = results.into_iter().sum();

    println!("{total}");
}

fn main() {
    println!("DAY 1");
    day1();

    println!("DAY 2");
    day2();

    println!("DAY 3");
    day3();
}
