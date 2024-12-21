// https://www.codewars.com/kata/55dc4520094bbaf50e0000cb

// solution 1: 812ms
function amIWilson(p) {
    let m = p * p;
    return (mod_factorial(p-1, m) + 1) % m === 0;
}

function mod_factorial(n, m) {
    let result = 1;
    for (i = 1; i <= n; i++) {
        result = (result * i) % m;
    }
    return result;
}

// solution 2:
// the only known Wilson primes are 5, 13, 563
function amIWilson(p) {
    return [5, 13, 563].indexOf(p) > -1;
}