use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input");
    let mut a_counter = 0;
    let mut b_counter = 0;

    for line in input.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|d| d.parse::<i32>().ok())
            .collect();

        if safe(&report) {
            a_counter += 1;
        }

        if permissive_safe(&report) {
            b_counter += 1;
        }
    }
    println!("part a: {}", a_counter);
    println!("part b: {}", b_counter);
}

fn safe(report: &[i32]) -> bool {
    let init_sign = (report[0] - report[1]).signum();

    report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(current, next)| current - next)
        .all(|diff| diff.abs() > 0 && diff.abs() < 4 && diff.signum() == init_sign)
}

fn permissive_safe(report: &[i32]) -> bool {
    if safe(report) {
        true
    } else {
        if safe(&report[1..report.len()]) {
            return true;
        }

        let init_sign = (report[0] - report[1]).signum();

        //starting index of the problematic comparison
        //the problematic index is either i or i+1
        let i = report
            .iter()
            .zip(report.iter().skip(1))
            .map(|(current, next)| current - next)
            .position(|diff| diff.abs() == 0 || diff.abs() > 3 || diff.signum() != init_sign);

        if i.is_some() {
            let i = i.unwrap();

            if safe(&[&report[0..i], &report[i + 1..report.len()]].concat()) {
                return true;
            }

            if safe(&[&report[0..i + 1], &report[i + 2..report.len()]].concat()) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {

    //https://www.reddit.com/r/adventofcode/comments/1h4shdu/2024_day_2_part2_edge_case_finder/
    const TEST: &str = "48 46 47 49 51 54 56
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
        for l in TEST.lines() {
            let report: Vec<i32> = l
                .split_whitespace()
                .filter(|str| str.parse::<i32>().is_ok())
                .map(|d| d.parse::<i32>().unwrap())
                .collect();

            if super::permissive_safe(&report) {
                counter += 1;
            }
        }
        assert_eq!(counter, 10);
    }
}
