use itertools::{repeat_n, Itertools};
use std::fs;

enum Operations {
    MUL,
    SUM,
    CAT,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let input = parse(&input);

    let a_operators = [Operations::MUL, Operations::SUM];
    let a_total: u64 = input
        .iter()
        .filter_map(|(test_val, eq_vals)| solve(*test_val, eq_vals, &a_operators))
        .sum();
    println!("Part 1: {}", a_total);

    let b_operators = [Operations::MUL, Operations::SUM, Operations::CAT];
    let b_total: u64 = input
        .iter()
        .filter_map(|(test_val, eq_vals)| solve(*test_val, eq_vals, &b_operators))
        .sum();
    println!("Part 2: {}", b_total);
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|s| {
            let (test_val, equation) = s.split_once(":").unwrap();
            let test_val: u64 = test_val.parse().unwrap();
            let eq_vals: Vec<u64> = equation
                .split(" ")
                .filter_map(|val| val.parse().ok())
                .collect();

            (test_val, eq_vals)
        })
        .collect()
}

fn solve(test_val: u64, eq_vals: &[u64], operators: &[Operations]) -> Option<u64> {
    let n = eq_vals.len() - 1;
    let operations_iter = repeat_n(operators.iter(), n).multi_cartesian_product();

    for operations in operations_iter {
        let mut result: u64 = eq_vals[0];

        for (i, val) in eq_vals.iter().enumerate().skip(1) {
            match operations[i - 1] {
                Operations::SUM => result += *val,
                Operations::MUL => result *= *val,
                Operations::CAT => {
                    result = concat(result, *val);
                }
            }
        }

        if result == test_val {
            return Some(test_val);
        }
    }

    None
}

fn concat(x: u64, y: u64) -> u64 {
    let offset: u64 = 10u64.pow((y.checked_ilog10().unwrap_or(0) + 1).into());

    x * offset + y
}
