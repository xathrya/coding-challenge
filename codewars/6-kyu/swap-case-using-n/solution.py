# https://www.codewars.com/kata/5f3afc40b24f090028233490

# solution 1: 574ms
def swap(s, n):
    if s == "" or n == 0:
        return s
    
    mask = f"{n:b}"
    result = []
    ctr = 0
    for c in s:
        if not c.isalpha():
            result.append(c)
        else:
            if mask[ctr] == "1":
                result.append(c.swapcase())
            else:
                result.append(c)
            ctr = (ctr + 1) % len(mask)
    
    return "".join(result)


# solution 2: 641ms
from itertools import cycle

def swap(s, n):
    mask = cycle(bin(n)[2:])
    return "".join(c.swapcase() if c.isalpha() and next(mask) == '1' else c for c in s)