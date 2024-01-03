// https://www.codewars.com/kata/541c8630095125aba6000c00

// solution 1: 1225ms
fn digital_root(n: i64) -> i64 {
    let mut number: i64 = num;

    loop {
        // break the digits into vector of digits
        let digits: Vec<i64> = number 
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i64)
            .collect();
        
        // sum the digits
        number = digits.iter().sum();

        // match the length of digit
        match digits.len() {
            2.. => continue, 
            _   => return number,
        }
    }
}


// solution 2: 
fn digital_root(n: i64) -> i64 {
    if n < 10 {
        n 
    } else {
        digital_root(sum_digits(n))
    }
}

fn sum_digits(n: i64) -> i64 {
    let mut sum = 0;
    let mut n = n;

    while n != 0 {
        sum += n % 10;
        n = n / 10;
    }

    sum 
}


// solution 3: 
fn digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}