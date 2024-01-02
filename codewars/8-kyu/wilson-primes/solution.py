# https://www.codewars.com/kata/55dc4520094bbaf50e0000cb

# solution 1: 547ms
def am_i_wilson(n):
    if (n < 2) or (n % 2 == 0) or not all(n % i for i in range(3, int(n ** 0.5) + 1, 2)):
        return False
    
    return (mod_factorial(n - 1) + 1) % (n ** 2) == 0

def mod_factorial(n, m):
    result = 1
    for i in range(1, n+1):
        result = (result * i) % m 
    
    return result 

# solution 2:
# the only known wilson primes are 5, 13, 563
def am_i_wilson(n):
    return n in (5, 13, 563)