// https://www.codewars.com/kata/576bb71bbbcf0951d5000044

// solution 1: 1409ms
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut pos: i32 = 0;
    let mut neg: i32 = 0;
    
    if input.len() == 0 {
        return vec![];
    }
    
    for i in input {
        if i <= 0 { neg += i; } 
        else { pos += 1; }
    }
    
    vec![pos, neg]
}


// solution 2: 
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    input.iter().fold(vec![0, 0], |mut acc, &x| {
        if x > 0 {
            acc[0] += 1;
        } else {
            acc[1] += x;
        }
        acc
    })
}


// solution 3:
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    vec![
        input.iter().filter(|&&x| x > 0).count() as i32,
        input.iter().filter(|&&x| x < 0).sum()
    ]
}