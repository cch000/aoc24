use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let mut a_counter = 0;
    let mut b_counter = 0;

    for l in input.lines() {
        let digits: Vec<i32> = l
            .split_whitespace()
            .filter(|str| str.parse::<i32>().is_ok())
            .map(|d| d.parse::<i32>().unwrap())
            .collect();

        if safe(&digits) {
            a_counter += 1;
        }

        if permissive_safe(&digits) {
            b_counter += 1;
        }
    }

    println!("part a: {}", a_counter);
    println!("part b: {}", b_counter);
}

fn permissive_safe(digits: &Vec<i32>) -> bool {
    if safe(digits) {
        true
    } else {
        for i in 0..digits.len() {
            let mut perm_digits = digits.clone();

            perm_digits.remove(i);

            if safe(&perm_digits) {
                return true;
            }
        }
        false
    }
}

fn safe(digits: &[i32]) -> bool {
    let init_sign = (digits[0] - digits[1]).signum();

    let not_safe = digits
        .iter()
        .zip(digits.iter().skip(1))
        .map(|(d, next)| d - next)
        .any(|diff| diff.abs() == 0 || diff.abs() > 3 || diff.signum() != init_sign);

    !not_safe
}

#[cfg(test)]
mod test {

    //https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/
    const REPORT: &str = "48 46 47 49 51 54 56
    1 1 2 3 4 5
    1 2 3 4 5 5
    5 1 2 3 4 5
    1 4 3 2 1
    1 6 7 8 9
    1 2 3 4 3
    9 8 7 6 7
    7 10 8 10 11
    29 28 27 25 26 25 22 20";

    #[test]
    fn test() {
        let mut counter = 0;
        for l in REPORT.lines() {
            let digits: Vec<i32> = l
                .split_whitespace()
                .filter(|str| str.parse::<i32>().is_ok())
                .map(|d| d.parse::<i32>().unwrap())
                .collect();

            if super::permissive_safe(&digits) {
                counter += 1;
            }
        }
        assert_eq!(counter, 10);
    }
}