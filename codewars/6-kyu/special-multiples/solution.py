# https://www.codewars.com/kata/55e785dfcb59864f200000d9

# solution 1: 820ms
import math 

def prime(n, primes):
    for prime in primes:
        if n % prime == 0:
            return False 
    primes.add(n)
    return True

def first_primes(n):
    primes = set([2])
    i, p = 2, 1
    while p < n:
        if prime(i, primes):
            p += 1
        i += 1
    return primes

def count_specMult(n, max_val):
    num = math.prod(first_primes(n))
    return len(list(range(num, max_val+1, num)))