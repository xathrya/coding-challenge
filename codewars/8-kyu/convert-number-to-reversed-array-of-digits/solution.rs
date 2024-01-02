// https://www.codewars.com/kata/5583090cbe83f4fd8c000051

// solution 1: 1393ms
fn digitize(n: u64) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut n = n;
    
    if n == 0 {
        result.push(0);
    } else {
        while n > 0 {
            result.push((n % 10) as u8);
            n = n / 10;
        }
    }
    result
}

// solution 2: 1840ms
fn digitize(n: u64) -> Vec<u8> {
    n   .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}