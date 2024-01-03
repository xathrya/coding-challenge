// https://www.codewars.com/kata/56bdd0aec5dc03d7780010a5

// solution 1: 1489ms
fn next_higher(n: i32) -> i32 {
    let t = n | (n - 1);
    (t + 1) | ((!t & -!t) - 1) >> (n.trailing_zeros() +1)
}

// solution 2: 1551ms
fn next_higher(n: i32) -> i32 {
    let pos = n & (-n);
    let val = n + pos;
    val | ((n ^ val) / (pos << 2))
}