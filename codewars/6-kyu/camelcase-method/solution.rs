// https://www.codewars.com/kata/587731fda577b3d1b0001196

// solution 1: 1583ms
fn camel_case(s: &str) -> String {
    s.split_ascii_whitespace()
        .map(|w| w[..1].to_ascii_uppercase() + &*w[1..].to_ascii_lowercase())
        .collect()
}