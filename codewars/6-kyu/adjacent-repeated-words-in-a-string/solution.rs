// https://www.codewars.com/kata/5245a9138ca049e9a10007b8

// solution 1: 1567ms
fn count_adjacent_pairs(search_string: &str) -> usize {
    let mut res = 0;
    let mut prev_word: &[u8] = &[];
    let mut prev_eq = false;
    for word in search_string.as_bytes().split(|&b| b == b' ') {
        if !word.is_empty() && word.eq_ignore_ascii_case(prev_word) {
            if !prev_eq {
                res += 1;
            }
            prev_eq = true;
        } else {
            prev_eq = false;
        }
        prev_word = word;
    }
    res
}


// solution 2:
use itertools::Itertools;

fn count_adjacent_pairs(search_string: &str) -> usize {
    search_string
        .to_lowercase()
        .split_whitespace()
        .group_by(|&word| word)
        .into_iter()
        .map(|(_, g)| g.count)
        .filter(|&c| c > 1)
        .count()
}

// solution 3:
fn count_adjacent_pairs(search_string: &str) -> usize {
    search_string
        .split_whitespace()
        .zip(search_string.split_whitespace().skip(1))
        .fold((0, None), |(count, last), (a,b)| {
            match last != Some(a) && a.eq_ignore_ascii_case(b) {
                true    => (count + 1, Some(a)),
                false   => (count, Some(a)),
            }
        }).0
}