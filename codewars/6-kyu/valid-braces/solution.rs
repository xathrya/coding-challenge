// https://www.codewars.com/kata/5277c8a221e209d3f6000b56

// solution 1: 1320ms
fn valid_braces(s: &str) -> bool {
    let braces = vec!["{}", "()", "[]"];
    if !braces.iter().any(|&x| s.contains(x)) {
        return false;
    }
    let mut result = "";
    for b in braces {
        if s.contains(b) {
            result = b;
            break;
        }
    }
    if s == result {
        return true;
    } else {
        return valid_braces(s.replace(result, "").as_str());
    }
}

// solution 2: 1585ms
fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            ')' => if stack.pop() != Some('(') { return false; },
            '{' => stack.push(c),
            '}' => if stack.pop() != Some('{') { return false; },
            '[' => stack.push(c),
            ']' => if stack.pop() != Some('[') { return false; },
            _   => panic!("invalid input"),
        }
    }
    stack.is_empty()
}


// solution 3: 1150ms
fn valid_braces(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            x   => if Some(x) != stack.pop() { return false },
        }
    }
    stack.is_empty()
}