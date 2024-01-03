// https://www.codewars.com/kata/563f0c54a22b9345bf000053

// solution 1: 
fn fcn(n: i32) -> i64 {
    1 << n 
}

// solution 2:
fn fcn(n: i32) -> i64 {
    2i64.pow(n as u32)
}