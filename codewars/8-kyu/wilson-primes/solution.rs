// https://www.codewars.com/kata/55dc4520094bbaf50e0000cb

// solution 1: 2485ms
// the only known wilson primes are 5, 13, 563
fn am_i_wilson(n: u32) -> bool {
    [5, 13, 563].contains(&n)
}