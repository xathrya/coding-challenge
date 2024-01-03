# https://www.codewars.com/kata/factorial-length
# ref: https://oeis.org/A034886

# solution 1: 678ms
import math 
def count(n):
    return int(math.ceil(math.log(2 * math.pi * n, 10) / 2 + n * math.log(n / math.e, 10)))


# solution 2: 510ms
import math 
def count(n):
    return math.ceil(math.lgamma(n+1)/math.log(10))