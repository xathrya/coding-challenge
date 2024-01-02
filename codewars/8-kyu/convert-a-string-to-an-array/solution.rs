// https://www.codewars.com/kata/57e76bc428d6fbc2d500036d

// solution 1: 1815ms
fn string_to_array(s: &str) -> Vec<String> {
    Vec::from_iter(s.split_whitespace().map(String::from))
}


// solution 2: 1740ms
fn string_to_array(s: &str) -> Vec<String> {
    s.split_whitespace().map(str::to_string).collect()
}


// solution 3: 1502ms
fn string_to_array(s: &str) -> Vec<String> {
    s.split_whitespace().map(|x| x.to_string()).collect()
}