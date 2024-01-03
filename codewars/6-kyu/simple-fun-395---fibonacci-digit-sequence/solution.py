# https://www.codewars.com/kata/5bc555bb62a4cec849000047

# solution 1: 493ms
def find(a, b, n):
    s = str(a) + str(b)

    if n > 20:
        n = n%20 + 20
    
    while len(s) <= n:
        ch = int(s[-1]) + int(s[-2])
        s = s + str(ch)
    
    return int(s[n])