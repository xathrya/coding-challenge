# https://www.codewars.com/kata/5a58d46cfd56cb4e8600009d

# solution 1: 486ms
def halving_sum(n): 
    s = n
    while n > 1:
        n = n // 2
        s += n
    return s