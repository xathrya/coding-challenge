// https://www.codewars.com/kata/57cfdf34902f6ba3d300001e

// solution 1: 1493ms
fn two_sort(arr: &[&str]) -> String {
    arr.iter()
        .min()
        .unwrap()
        .chars()
        .fold(String::new(), |acc, s| format!("{}***{}", acc, s))[3..]
        .to_string()
}