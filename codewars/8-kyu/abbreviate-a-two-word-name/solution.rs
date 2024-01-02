// https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3

// solution 1: 1378ms
fn abbrev_name(name: &str) -> String {
    let mut result = String::new();
    let arr: Vec<&str> = name.split_whitespace().collect();
    
    result.push_str(&arr[0][..1].to_uppercase());
    result.push_str(".");
    result.push_str(&arr[1][..1].to_uppercase());
    
    result
}

// solution 2: 1534ms
fn abbrev_name(name: &str) -> String {
    name.split(' ')
      .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
      .collect::<Vec<_>>()
      .join(".")
}

// solution 3: 1530ms
fn abbrev_name(name: &str) -> String {
    name.split(' ')
      .map(|word| word[0..1].to_uppercase())
      .collect::<Vec<_>>()
      .join(".")
}