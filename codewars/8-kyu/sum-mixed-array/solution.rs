// https://www.codewars.com/kata/57eaeb9578748ff92a000009

// solution 1: 1426ms
use either::Either;

fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    arr .iter()
        .map(|val| {
            match val {
                Either::Left(n)     => *n,
                Either::Right(s)    => s.parse::<i32>().unwrap(),
            }
        })
        .sum()
}


// solution 2: 1368ms
use either::Either;

fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    arr .iter()
        .cloned()
        .map(|val| val.left_or_else(|x| x.parse::<i32>().unwrap()))
        .sum()
}