// https://www.codewars.com/kata/56efc695740d30f963000557

// solution 1: 1416ms
fn to_alternating_case(s: &str) -> String {
    s
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}