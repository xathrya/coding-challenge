// https://www.codewars.com/kata/5518a860a73e708c0a000027

// solution 1: 1394ms
fn last_digit(list: &[u64]) -> u64 {
    let f = |x,y| std::cmp::min(x % y + y, x);
    list.iter().rev().fold(1, |v, &n| f(n, 20).pow(f(v, 4) as u32)) % 10
}


// solution 2: 1469ms
fn mod_off(num: i32, mod_num: i32) -> i32 { num.min((num - 2) % mod_num + 2) }

fn pow_mod(exp: i32, base: i32) -> i32 { (mod_off(base, 20) as f64).powi(mod_off(exp, 4)) as i32 }

fn last_digit(lst: &[u64]) -> u64 { lst.iter().map(|x| *x as i32).rev().fold(1, pow_mod) as u64 % 10 }


// solution 2: 1681ms
const TABLE: [[u8; 4]; 10] = [
    [0, 0, 0, 0],
    [1, 1, 1, 1],
    [6, 2, 4, 8],
    [1, 3, 9, 7],
    [6, 4, 6, 4],
    [5, 5, 5, 5],
    [6, 6, 6, 6],
    [1, 7, 9, 3],
    [6, 8, 4, 2],
    [1, 9, 1, 9],
];

fn evals_to_zero(list: &[u64]) -> bool {
    list.iter().take_while(|&&x| x == 0).count() % 2 == 1
}

pub fn last_digit(list: &[u64]) -> u64 {
    match list {
        [] => 1,
        [_, tail @ ..] if evals_to_zero(tail) => 1,
        [a, tail @ ..] => {
            let exp_mod_4 = match tail {
                [] => 1,
                [_, tail @ ..] if evals_to_zero(tail) => 1,
                [b] => (b % 4) as u8,
                [b, c, tail @ ..] => match b % 4 {
                    0 => 0,
                    1 => 1,
                    2 => {
                        if *c == 1 || evals_to_zero(tail) {
                            2
                        } else {
                            0
                        }
                    }
                    3 => {
                        if c % 2 == 0 && !evals_to_zero(tail) {
                            1
                        } else {
                            3
                        }
                    }
                    _ => unreachable!(),
                },
            };
            TABLE[(a % 10) as usize][exp_mod_4 as usize] as _
        }
    }
}