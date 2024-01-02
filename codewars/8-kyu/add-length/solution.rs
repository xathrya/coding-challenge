// https://www.codewars.com/kata/559d2284b5bb6799e9000047

// solution 1: 1670ms
fn add_length(s: &str) -> Vec<String> {
    let mut res = Vec::with_capacity(s.len() / 2 + 1);
    for word in s
        .as_bytes()
        .split(|&b| b == b' ')
        .map(|word| unsafe { str::from_utf8_unchecked(word) })
    {
        let entry = format!("{} {}", word, word.len());
        res.push(entry);
    }
    
    res
}

// solution 2: 1660ms
fn add_length(s: &str) -> Vec<String> {
    s.split_whitespace().map(|x| format!("{} {}", x, x.len())).collect()
}

