// https://www.codewars.com/kata/54ff3102c1bad923760001f3

// solution 1: 1330ms
fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    for c in string.chars() {
        if vowels.contains(&c) {
            vowels_count += 1;
        }
    }

    vowels_count
}

// solution 2: 1372ms
fn get_count(string: &str) -> usize {
    string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count()
}

// solution 3: 1390ms
fn get_count(string: &str) -> usize {
    string.chars().filter(|&c| "aeiou".contains(c)).count()
}