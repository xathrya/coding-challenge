// https://www.codewars.com/kata/5541f58a944b85ce6d00006a

// solution 1: 1120ms
fn product_fib(prod: u64) -> (u64, u64, bool) {
    let mut a: u64 = 1;
    let mut b: u64 = 1;

    while a*b < prod {
        let t = a + b;
        a = b;
        b = t;
    }

    (a, b, a*b == prod)
}