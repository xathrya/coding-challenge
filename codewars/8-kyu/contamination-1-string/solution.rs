// https://www.codewars.com/kata/596fba44963025c878000039

// solution 1: 1641ms
fn contamination(text: &str, character: &str) -> String {
    character.repeat(text.len())
}
