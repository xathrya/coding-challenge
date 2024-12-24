# https://www.codewars.com/kata/544d114f84e41094a9000439

# solution 1: 490ms
def powerof4(n):
    return type(n)==int and n >= 0 and log(n, 4).is_integer()


# solution 2: 447ms
from math import log
def powerof4(n):
    if type(n) in (int, float) and n > 0:
        return log(n, 4).is_integer()
    
    return False


# solution 3: 604ms
def powerof4(n):
    if not type(n)==int: return False 
    if n <= 0: return False
    while n % 4 == 0:
        n = n // 4
    
    return n == 1