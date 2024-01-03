# https://www.codewars.com/kata/541c8630095125aba6000c00
# digital root: https://en.wikipedia.org/wiki/Digital_root

# solution 1: 497ms
def digital_root(n):
    n = str(n)
    while len(n) > 1:
        n = str(sum(map(int, n)))
    return int(n)

# solution 2: 479ms
def digital_root(n):
    if n < 10:
        return n 
    else:
        return digital_root(sum(map(int, str(n))))