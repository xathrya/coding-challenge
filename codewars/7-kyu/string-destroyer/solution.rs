// https://www.codewars.com/kata/5872637c2eefcb1216000081

// solution 1: 2071ms
use std::collections::HashSet;

fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    let mut base_str = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    for input_set in input_sets {
        for i in input_set {
            if i != '_' && i != ' ' {
                base_str = base_str.replace(i, "_");
            }
        }
    }

    base_str
}


// solution 2: 
use std::collections::HashSet;

fn destroy(input_sets: Vec<HashSet<char>>) -> String {
    "a b c d e f g h i j k l m n o p q r s t u v w x y z"
        .chars()
        .map(|c| if c != ' ' && input_sets.iter().any(|s| s.contains(&c)) {'_'} else {c})
        .collect()    
}