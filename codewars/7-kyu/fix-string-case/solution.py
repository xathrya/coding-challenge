# https://www.codewars.com/kata/5b180e9fedaa564a7000009a

# solution 1: 659ms
def solve(s):
    n_lower, n_upper = 0, 0
    for c in s:
        if c.isupper(): 
            n_upper += 1
        elif c.islower():
            n_lower += 1
    
    if n_upper > n_lower:
        return s.upper()
    else:
        return s.lower()


# solution 2: 510ms
def solve(s):
    return s.upper() if sum(map(str.isupper, s)) * 2 > len(s) else s.lower()