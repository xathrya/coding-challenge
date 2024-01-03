# https://www.codewars.com/kata/570409d3d80ec699af001bf9

# solution 1: 498ms
def fusc(n):
    assert type(n) == int and n >= 0
    
    if n < 2:
        return n
    elif n % 2 == 0:
        return fusc(n//2)
    else:
        return fusc(n//2) + fusc(n//2 + 1)