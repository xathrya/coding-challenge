// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0

// solution 1: 1408ms
pub fn remove_char(s: &str) -> String {
    let mut s = s.chars();
    s.next();
    s.next_back();
    s.as_str().to_string()
}


// solution 2: 1403ms
pub fn remove_char(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}


// solution 3: 1303ms
pub fn remove_char(s: &str) -> String {
    let mut s = s.to_string();
    s.pop();
    s.remove(0);
    s
}


// solution 4: 1676ms
pub fn remove_char(s: &str) -> String {
    s.chars().skip(1).take(s.chars().count() - 2).collect()
}