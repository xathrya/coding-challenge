// https://www.codewars.com/kata/5513795bd3fafb56c200049e

// solution 1: 1458ms
fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).map(|i| i*x).collect::<Vec<u32>>()
}