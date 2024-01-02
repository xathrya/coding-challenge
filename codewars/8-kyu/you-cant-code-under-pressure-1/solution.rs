// https://www.codewars.com/kata/53ee5429ba190077850011d4

// solution 1: 1057ms
fn double_integer(n: i32) -> i32 {
    n * 2
}

// solution 2: 1535ms
fn double_integer(n: i32) -> i32 {
    n<<1
}

// solution 3: 1195ms
fn double_integer(n: i32) -> i32 {
    n + n
}