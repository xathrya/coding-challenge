// https://www.codewars.com/kata/57f780909f7e8e3183000078

// 1165ms
fn grow(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(1, |acc, x| acc * x)
}