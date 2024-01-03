// https://www.codewars.com/kata/5949481f86420f59480000e7

// solution 1: 1264ms
fn odd_or_even(numbers: Vec<i32>) -> String {
    (if numbers.iter().sum::<i32>() % 2 == 0 { "even" } else { "odd" }).to_string()
}

// solution 2: 
fn odd_or_even(numbers: Vec<i32>) -> String {
    ["even", "odd"][numbers.iter().sum::<i32>() as usize % 2].to_string()
}