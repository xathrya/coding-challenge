// https://www.codewars.com/kata/54b724efac3d5402db00065e

mod preloaded;
use preloaded::MORSE_CODE;
// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

// solution 1: 1935ms
fn decode_morse(encoded: &str) -> String {
    encoded.split("   ").map(|word| {
        word.split_whitespace().map(|ch| {
            MORSE_CODE.get(ch).unwrap().to_string()
        })
        .collect::<String>()
    })
    .collect::<Vec<String>>()
    .join(" ")
    .trim()
    .to_string()
}