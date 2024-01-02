// https://www.codewars.com/kata/55a2d7ebe362935a210000b2

// solution 1: 1308ms
fn find_smallest_int(arr: &[i32]) -> i32 {
    *arr.iter().min().unwrap()
}