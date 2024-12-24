// https://www.codewars.com/kata/5ac6932b2f317b96980000ca


// solution 1: 2050ms
fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort_unstable_by(|a, b| b.cmp(a));
    digits.dedup();
    digits
        .iter()
        .enumerate()
        .map(|(i, d)| 10i32.pow(i as u32) * d)
        .sum()
}


// solution 2: 1700ms
fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    digits
        .into_iter()
        .fold(0, |acc, d| acc * 10 + d)
}


// solution 3: 1673ms
fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();
    digits
        .iter()
        .map(|d| d.to_string())
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}