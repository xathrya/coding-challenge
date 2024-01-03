// https://www.codewars.com/kata/55be10de92aad5ef28000023

// solution 1: 1138ms
fn check_choose(m: u64, n: u64) -> i64 {
    let mut result: u64 = 1;
    for i in 0..=n {
        if result == m {
            return i as i64;
        }
        result = result * (n - i) / (i + 1);
    }
    -1
}


// solution 2: 1555ms
fn check_choose(m: u64, n: u64) -> i64 {
    let mut result: i64 = -1;
    let h = (n + 2) / 2;

    for x in 1..h {
        if get_choices(n, x) == m {
            result = x as i64;
            break;
        }
    }
    result
}

fn get_choices(n: u64, x: u64) -> u64 {
    fn get_choices_iter(n:u64, x: u64, c: u64, r: u64) -> u64 {
        let r = r * (n - c + 1) / c;
        if c < x {
            get_choices_iter(n, x, c + 1, r)
        } else {
            r
        }
    }
    get_choices_iter(n, x, 1, 1)
}

