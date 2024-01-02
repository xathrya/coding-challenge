// https://www.codewars.com/kata/5a00e05cc374cb34d100000d

// solution 1: 1620ms
fn reverse_seq(n: u32) -> Vec<u32> {
    (1..n+1).rev().collect()
}