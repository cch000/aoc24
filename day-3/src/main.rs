use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("error reading input");
    println!("Total a: {}", part_a(&input));
    println!("Total b: {}", part_b(&input));
}

fn part_a(input: &str) -> i32 {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|capture| {
            let (_, [x, y]) = capture.extract();
            x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap()
        })
        .sum()
}

fn part_b(input: &str) -> i32 {
    let mut enabled = true;

    Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))")
        .unwrap()
        .captures_iter(input)
        .filter_map(|capture| {
            capture.get(4).is_some().then(|| enabled = false);
            capture.get(3).is_some().then(|| enabled = true);

            if enabled {
                let b: Option<i32> = capture
                    .get(2)
                    .is_some()
                    .then(|| capture[2].parse().unwrap());
                let a: Option<i32> = capture
                    .get(1)
                    .is_some()
                    .then(|| capture[1].parse().unwrap());

                (a.is_some() && b.is_some()).then(|| a.unwrap() * b.unwrap())
            } else {
                None
            }
        })
        .sum()
}
