// https://www.codewars.com/kata/5700c9acc1555755be00027e/

// solution 1: 1580ms
fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool { 
    let mutation = strng.repeat(2);
    let len = strng.len();
    
    (0..len).all(|offset| arr.contains(&&mutation[offset..offset+len]))
}

// solution 2: 1651ms
fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool { 
    (0..strng.len())
        .map(|index| strng[index..].to_owned() + &strng[..index])
        .all(|x| arr.contains(&x.as_str()))
}

// solution 3: 1826ms
use std::collections::HashSet;
fn contain_all_rots(strng: &str, arr: Vec<&str>) -> bool { 
    let set : HashSet<String> = arr.iter().map(|s| s.to_string()).collect();
    let mut rots = (0..strng.len()).map(|i| strng[i..strng.len()].to_string() + &strng[0..i]);

    return rots.all(|s| set.contains(&s))
}