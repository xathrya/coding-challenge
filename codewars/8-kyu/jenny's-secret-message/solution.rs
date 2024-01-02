// https://www.codewars.com/kata/55225023e1be1ec8bc000390

// solution 1: 1473ms
fn greet(input : &str) -> String {
    match input {
        "Johnny" => "Hello, my love!".to_string(),
        _ => format!("Hello, {}!", input),
    }
}