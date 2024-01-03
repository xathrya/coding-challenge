# https://www.codewars.com/kata/5a908da30025e995880000e3

# solution 1: 8139ms
def solve(a, b):
    c = prime_series(round((a + b)*2.12))
    return c[a:a+b]

def prime_series(number):
    is_prime = lambda number: all( number%i != 0 for i in range(2, int(number**.5)+1) )
    ret = ''
    for n in range(2, number):
        if is_prime(n) == True:
            ret += str(n)      
    return ret