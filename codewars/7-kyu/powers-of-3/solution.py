# https://www.codewars.com/kata/57be674b93687de78c0001d9

# solution 1: 497ms
from math import log, ceil
def largest_power(N):
    return ceil(log(N, 3)) - 1


# solution 2: 499ms
def largest_power(N):
    k = 0
    while 3 ** k < N:
        k += 1
    
    return k