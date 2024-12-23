// https://www.codewars.com/kata/55c45be3b2079eccff00010f


// solution 1: 2091ms
fn order(sentence: &str) -> String {
    let mut words: Vec<_> = sentence.split_ascii_whitespace().map(String::from).collect();
    words.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    words.join(" ")
}

// solution 2: 1809ms
fn order(sentence: &str) -> String {
    let mut words = sentence.split_ascii_whitespace().collect::<Vec<_>>();
    words.sort_by(|a, b| digit(a).cmp(&digit(b)));
    words.join(" ")
}

fn digit(word: &str) -> u32 {
    let mut result = 0;
    for c in word.chars() {
        if c.is_ascii_digit() {
            result = c.to_digit(10).unwrap();
        }
    }
    result
}