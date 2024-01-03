# https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec

# solution 1: 524ms
import math
def persistence(n):
    n = str(n)
    count = 0
    while len(n) > 1:
        n = str(math.prod(map(int, n)))
        count += 1
    return count

# solution 2: 476ms
from functools import reduce 
from operator import mul
def persistence(n):
    n = str(n)
    count = 0
    while len(n) > 1:
        n = str(reduce(mul, map(int, n)))
        count += 1
    return count

# solution 3: 658ms
from functools import reduce
def persistence(n):
    n = str(n)
    count = 0
    while len(n) > 1:
        n = str(reduce(lambda x,y: x*y, map(int, n)))
        count += 1
    return count 