use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read file");
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for l in input.lines() {
        let mut parts = l.split_whitespace();
        left.push(parts.next().unwrap().parse::<u32>().unwrap());
        right.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    a(&mut left, &mut right);
    b(&left, &right);
}

fn a(left: &mut Vec<u32>, right: &mut Vec<u32>) {
    left.sort();
    right.sort();

    let sum: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("a: {}", sum)
}

fn b(left: &Vec<u32>, right: &Vec<u32>) {
    let sum: u32 = left
        .iter()
        .map(|&n| n * right.iter().filter(|&&x| x == n).count() as u32)
        .sum();

    println!("b: {}", sum);
}
