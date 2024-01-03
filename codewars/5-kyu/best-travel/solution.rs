// https://www.codewars.com/kata/55e7280b40e1c4a06d0000aa

// solution 1: 1582ms
fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    if (ls.len() >= std::mem::size_of::<usize>() * 8) || (k as usize > ls.len()) {
        return -1;
    }
    
    let mut best = -1;
    
    for i in 1..(1usize << ls.len()) {
        if i.count_ones() == k as u32 {
            let mut sum = 0;
            
            for j in 0..ls.len() {
                if ((i >> j) & 0x1) != 0 {
                    sum += ls[j];
                }
            }
            
            if sum <= t && sum > best {
                best = sum;
            }
        }
    }
    best
}

// solution 2: 1493ms
extern crate itertools;
use itertools::Itertools;
fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    ls 
        .iter()
        .combinations(k as usize)
        .map(|comb| comb.into_iter().sum())
        .filter(|s| s <= &t)
        .max()
        .unwrap_or(-1)
}