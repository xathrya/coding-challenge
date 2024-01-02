// https://www.codewars.com/kata/523b623152af8a30c6000027

// solution 1: 1697ms
fn square(n: i32) -> i32 {
    n * n 
}

// solution 2: 1197ms
fn square(n: i32) -> i32 {
    n.pow(2)
}