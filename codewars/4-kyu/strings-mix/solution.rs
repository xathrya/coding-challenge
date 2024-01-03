// https://www.codewars.com/kata/5629db57620258aa9d000014

// solution 1: 1684ms
use std::collections::{HashMap, HashSet};

struct Map {
    prefix: char,
    order:  usize,
    c:      char,
    count:  usize,
}

fn repeat_char(&c: &char, n: usize) -> String {
    (0..n).map(|_| c).collect()
}

fn mix(s1: &str, s2: &str) -> String {
    let mut result: Vec<Map> = vec![];
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for c in s1.to_owned().chars() {
        if c.is_lowercase() {
            let counter = map1.entry(c).or_insert(0);
            *counter += 1;
        }
    }

    for c in s2.to_owned().chars() {
        if c.is_lowercase() {
            let counter = map2.entry(c).or_insert(0);
            *counter += 1;
        }
    }

    let mut keys = HashSet::new();
    for &c in map1.keys().chain(map2.keys()) {
        keys.insert(c);
    }

    for c in keys {
        let n1 = map1.get(&c);
        let n2 = map2.get(&c);
        let count1 = match n1 {
            Some(&n) => n,
            None => 0,
        };
        let count2 = match n2 {
            Some(&n) => n,
            None => 0,
        };
        if count1 > 1 || count2 > 1 {
            if count1 > count2 {
                let test = Map{ prefix: '1', order: 1, c: c, count: count1 };
                result.push(test);
            } else if count1 < count2 {
                let test = Map{ prefix: '2', order: 2, c: c, count: count2 };
                result.push(test);
            } else {
                let test = Map{ prefix: '=', order: 3, c: c, count: count2 };
                result.push(test);
            }
        }
    }

    result.sort_by_key(|m| (-(m.count as i32), m.order, m.c));
    let mut new_result = vec![];
    for map in result {
        new_result.push(format!("{}:{}", map.prefix, repeat_char(&map.c, map.count)));
    }

    new_result.join("/")
}


// solution 2: 
use std::cmp::Ordering::*;

fn mix(s1: &str, s2: &str) -> String {
    let mut all = (b'a' .. b'z' + 1).map(|b| {
        let b = b as char;
        
        let cnt1 = s1.chars().filter(|&c| c == b).collect::<String>();
        let cnt2 = s2.chars().filter(|&c| c == b).collect::<String>();
        
        match cnt1.len().cmp(&cnt2.len()) {
            Less    => format!("2:{}", cnt2),
            Equal   => format!("=:{}", cnt2),
            Greater => format!("1:{}", cnt1),
        }
    })
    .filter(|x| x.len() > 3)
    .collect::<Vec<_>>();
    
    all.as_mut_slice().sort_unstable_by(|a, b| match b.len().cmp(&a.len()) {
        Equal => a.cmp(&b),
        p => p
    });
    
    all.join("/")
}


// solution 3:
use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::iter;

fn count(s: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in s.chars().filter(char::is_ascii_lowercase) {
        map.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }
    map.retain(|_, c| *c > 1);
    map
}

fn mix(s1: &str, s2: &str) -> String {
    let m1 = count(s1);
    let m2 = count(s2);
    let mut keys: Vec<_> =
        m1.keys().chain(m2.keys())
        .map(|c| {
            let c1 = m1.get(c).cloned().unwrap_or(0);
            let c2 = m2.get(c).cloned().unwrap_or(0);
            let s = match c1.cmp(&c2) {
                Greater => '1',
                Equal => '=',
                Less => '2'
            };
            (-c1.max(c2), s, *c)
        })
        .collect();
    keys.sort_unstable();
    keys.dedup();
    keys.iter().map(|&(n, s, c)| {
        format!("{}:{}", s, iter::repeat(c).take(-n as usize).collect::<String>())
    }).collect::<Vec<_>>().join("/")
}