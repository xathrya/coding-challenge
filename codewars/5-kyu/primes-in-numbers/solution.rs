// https://www.codewars.com/kata/54d512e62a5e54c96200019e

// solution 1: 1343ms
fn prime_factors(n: i64) -> String {
    let mut number = n;
    let mut result = vec![];
    let mut factor = 2;
    let mut counter;

    while number > 1 {
        counter = 0;
        while number % factor == 0 {
            number /= factor;
            counter += 1;
        }

        if counter == 1 {
            result.push(format!("({})", factor));
        } else if counter > 1 {
            result.push(format!("({}**{})", factor, counter));
        }
        factor += 1;
    }

    result.join("")
}


// solution: 2116ms
use std::collections::BTreeMap;
fn prime_factors(n: i64) -> String {
    let mut result = BTreeMap::new();
    let mut prime = 2;
    let mut test = n;

    while prime <= test {
        if test % prime == 0 {
            test /= prime;
            let counter = result.entry(prime).or_insert(0);
            *counter += 1;
        } else {
            prime += 1;
        }
    }

    let mut output = String::from("");
    for (key, val) in result.iter() {
        if *val != 1 {
            output.push_str(format!("({}**{})", key, val).as_str());
        } else {
            output.push_str(format!("({})", key).as_str());
        }
    }

    output
}