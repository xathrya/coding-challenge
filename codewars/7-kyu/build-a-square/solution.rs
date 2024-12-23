// https://www.codewars.com/kata/59a96d71dbe3b06c0200009c

// solution 1: 1475ms
fn generate_shape(n: i32) -> String {
    let mut row = "+".repeat(n);
    row.push('\n');
    let mut square = row.repeat(n);
    square.pop();
    
    square
}


// solution 2: 1569ms
fn generate_shape(n: i32) -> String {
    vec!["+".repeat(n as usize); n as usize].join("\n")
}


// solution 3: 1423ms
fn generate_shape(n: i32) -> String {
    (1..=n)
        .map(|i| "+".repeat(n as usize))
        .collect::<Vec<String>>()
        .join("\n")
}