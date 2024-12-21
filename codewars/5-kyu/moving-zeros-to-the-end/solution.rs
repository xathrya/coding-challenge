// https://www.codewars.com/kata/52597aa56021e91c93000cb0

// solution 1: 1185ms
fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut result: Vec<_> = arr 
        .into_iter()
        .filter_map(|&x| if x != 0 { Some(x) } else { None })
        .collect();
    (0..arr.len() - result.len())
        .for_each(|_| result.push(0));

    result
}


// solution 2: 2120ms
use std::iter;
fn move_zeros(arr: &[u8]) -> Vec<u8> {
    arr .iter()
        .cloned()
        .filter(|&x| x != 0)
        .chain(iter::repeat(0))
        .take(arr.len())
        .collect()
}


// solution 3: 1918ms
fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity((arr.len()));
    result.extend(arr.iter().filter(|&&x| x != 0));
    result.resize(arr.len(), 0);
    result
}