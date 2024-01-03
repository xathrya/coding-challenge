// https://www.codewars.com/kata/546f922b54af40e1e90001da

// solution 1: 1524ms
use itertools::Itertools;

const C: u32 = ('a' as u32) - 1;

fn alphabet_position(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_ascii_alphabetic() {
                let num = c.to_ascii_lowercase() as u32 - C;
                Some(num.to_string())
            } else {
                None
            }
        })
        .join(" ")
}