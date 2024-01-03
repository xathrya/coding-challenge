// https://www.codewars.com/kata/5287e858c6b5a9678200083c

// solution 1: 1571ms
fn narcissistic(num: u64) -> bool {
    let s = num.to_string()
    let l = s.len() as u32;
    let mut result: u64 = 0;
    let mut number: u64 = num;
    
    while number > 0 {
        result += u64::pow(number % 10, l);
        number = number / 10;
    }
    num == result
}


// solution 2: 1637ms
fn narcissistic(num: u64) -> bool {
    let s = num.to_string();
    let l = s.len() as u32;

    num == (s.chars()
        .map(|d| d.to_digit(10).unwrap() as u64)
        .map(|d| d.pow(l)))
        .sum::<u64>()
}