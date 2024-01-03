// https://www.codewars.com/kata/56c1c1e4876de7e0cb000a10

// solution 1: 1112ms
fn previous_power_of_2(n: i32) -> i32 {
    if n == 1{ 0 }
    else if n > 0 {
        1 << (i32::BITS - 1 - (n - 1).leading_zeros())
    } else {
        -1 << (i32::BITS -(n - 1).leading_ones())
    }
}


// solution 2:
fn previous_power_of_2(n: i32) -> i32 {
    match n {
        2..=i32::MAX    => 1 << (i32::BITS - 1 - (n - 1).leading_zeros()),
        1               => 0,
        i32::MIN..=0    => -1 << (i32::BITS -(n - 1).leading_ones()),
    }
}
